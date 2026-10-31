/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
//! Stellar / Soroban Settlement Verifier & Non-Custodial Escrow State Machine
//! Validates on-chain cross-border payments, asset codes, and cryptographic pre-images.

use crate::crypto::Sha256;

/// Supported Stellar Assets for Cross-Border Nepal Remittance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StellarAsset {
    Xlm,
    Usdc { issuer: String },
    Eurc { issuer: String },
    Custom { code: String, issuer: String },
}

impl StellarAsset {
    pub const CIRCLE_USDC_ISSUER: &'static str =
        "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN";
    pub const CIRCLE_EURC_ISSUER: &'static str =
        "GD6WVYRVID442Y4JVWFWKWCZTRDGIIZM3FIQIJWJFD7WW7RUM2L7CJ2W";

    pub fn usdc() -> Self {
        Self::Usdc {
            issuer: Self::CIRCLE_USDC_ISSUER.to_string(),
        }
    }

    pub fn eurc() -> Self {
        Self::Eurc {
            issuer: Self::CIRCLE_EURC_ISSUER.to_string(),
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::Xlm => "XLM",
            Self::Usdc { .. } => "USDC",
            Self::Eurc { .. } => "EURC",
            Self::Custom { code, .. } => code.as_str(),
        }
    }

    pub fn is_stablecoin(&self) -> bool {
        matches!(self, Self::Usdc { .. } | Self::Eurc { .. })
    }
}

/// Memo attached to a Stellar payment used to link bank account transfer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StellarMemo {
    None,
    Text(String),
    Id(u64),
    Hash([u8; 32]),
}

/// On-chain proof of Stellar transaction verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StellarPaymentProof {
    pub tx_hash: String,
    pub source_account: String,
    pub destination_account: String,
    pub asset: StellarAsset,
    pub amount_stroops: u64, // 1 unit = 10,000,000 stroops (7 decimals)
    pub memo: StellarMemo,
    pub ledger_sequence: u64,
    pub confirmed_at_unix: u64,
}

impl StellarPaymentProof {
    /// Validates Stellar public key format (56 chars Base32, begins with 'G')
    pub fn is_valid_account_id(account_id: &str) -> bool {
        if account_id.len() != 56 || !account_id.starts_with('G') {
            return false;
        }
        account_id
            .chars()
            .all(|c| matches!(c, 'A'..='Z' | '2'..='7'))
    }

    /// Validates 64-character hex transaction hash
    pub fn is_valid_tx_hash(hash: &str) -> bool {
        hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Convert amount in stroops to decimal string representation
    pub fn amount_decimal(&self) -> String {
        let whole = self.amount_stroops / 10_000_000;
        let frac = self.amount_stroops % 10_000_000;
        format!("{}.{:07}", whole, frac)
    }
}

/// Non-Custodial Hash-Time Locked Contract (HTLC) Escrow State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Initiated,
    Funded,
    Claimed {
        preimage: String,
        settled_at_unix: u64,
    },
    Refunded {
        refunded_at_unix: u64,
    },
}

/// Non-custodial cross-border escrow contract state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemittanceEscrow {
    pub escrow_id: String,
    pub sender: String,
    pub recipient_nepal_bank: String,
    pub recipient_account: String,
    pub asset: StellarAsset,
    pub amount_stroops: u64,
    pub hash_lock: [u8; 32],
    pub timeout_unix: u64,
    pub status: EscrowStatus,
}

impl RemittanceEscrow {
    pub fn new(
        escrow_id: String,
        sender: String,
        recipient_nepal_bank: String,
        recipient_account: String,
        asset: StellarAsset,
        amount_stroops: u64,
        hash_lock: [u8; 32],
        timeout_unix: u64,
    ) -> Result<Self, &'static str> {
        if !StellarPaymentProof::is_valid_account_id(&sender) {
            return Err("Invalid sender Stellar account ID");
        }
        if amount_stroops == 0 {
            return Err("Amount must be greater than zero");
        }

        Ok(Self {
            escrow_id,
            sender,
            recipient_nepal_bank,
            recipient_account,
            asset,
            amount_stroops,
            hash_lock,
            timeout_unix,
            status: EscrowStatus::Initiated,
        })
    }

    pub fn mark_funded(&mut self) -> Result<(), &'static str> {
        if self.status != EscrowStatus::Initiated {
            return Err("Can only fund an initiated escrow");
        }
        self.status = EscrowStatus::Funded;
        Ok(())
    }

    /// Claim funds upon successful NCHL bank credit by presenting preimage
    pub fn claim(
        &mut self,
        preimage: &str,
        current_unix: u64,
    ) -> Result<(), &'static str> {
        if self.status != EscrowStatus::Funded {
            return Err("Escrow must be in Funded state to claim");
        }
        if current_unix > self.timeout_unix {
            return Err("Escrow timeout has expired; funds must be refunded to sender");
        }

        let computed_hash = Sha256::digest(preimage.as_bytes());
        if computed_hash != self.hash_lock {
            return Err("Invalid preimage: SHA-256 hash does not match hashlock");
        }

        self.status = EscrowStatus::Claimed {
            preimage: preimage.to_string(),
            settled_at_unix: current_unix,
        };
        Ok(())
    }

    /// Refund back to sender if NCHL settlement fails or timeout expires
    pub fn refund(&mut self, current_unix: u64) -> Result<(), &'static str> {
        if self.status != EscrowStatus::Funded {
            return Err("Escrow must be in Funded state to refund");
        }
        if current_unix <= self.timeout_unix {
            return Err("Cannot refund before timeout expiration");
        }

        self.status = EscrowStatus::Refunded {
            refunded_at_unix: current_unix,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_account_id() {
        assert!(StellarPaymentProof::is_valid_account_id(
            "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
        ));
        assert!(!StellarPaymentProof::is_valid_account_id("INVALID_ACCOUNT"));
        assert!(!StellarPaymentProof::is_valid_account_id(
            "BA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
        ));
    }

    #[test]
    fn test_htlc_escrow_lifecycle() {
        let secret = "nchl_secret_settlement_receipt_2026";
        let hash_lock = Sha256::digest(secret.as_bytes());
        let sender = "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN";
        let timeout = 1770000000;

        let mut escrow = RemittanceEscrow::new(
            "ESCROW-001".to_string(),
            sender.to_string(),
            "NABILNPKA".to_string(),
            "0010017500001".to_string(),
            StellarAsset::usdc(),
            1_000_000_000, // 100 USDC
            hash_lock,
            timeout,
        )
        .unwrap();

        assert_eq!(escrow.status, EscrowStatus::Initiated);
        escrow.mark_funded().unwrap();
        assert_eq!(escrow.status, EscrowStatus::Funded);

        // Attempt claim with wrong secret fails
        assert!(escrow.claim("wrong_secret", timeout - 100).is_err());

        // Correct secret succeeds
        escrow.claim(secret, timeout - 100).unwrap();
        assert!(matches!(escrow.status, EscrowStatus::Claimed { .. }));
    }
}
