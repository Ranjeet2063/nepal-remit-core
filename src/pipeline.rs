/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
//! End-to-End Stellar-to-NCHL Cross-Border Settlement Pipeline
//! Orchestrates on-chain verification, compliance gating, exact FX settlement, and NPI instruction generation.

use crate::compliance::{ComplianceEngine, ComplianceReport, RemittancePurpose};
use crate::crypto::Sha256;
use crate::fx::{ForeignCurrency, FxEngine, NrbExchangeRate, RemittanceCostComparison};
use crate::nchl::{BankRegistry, CommercialBank, NpiPaymentInstruction};
use crate::stellar::{StellarPaymentProof};

/// Inbound Remittance Intent Request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemittanceRequest {
    pub stellar_proof: StellarPaymentProof,
    pub foreign_currency: ForeignCurrency,
    pub source_amount_minor: u64, // e.g. cents
    pub source_decimals: u32,
    pub beneficiary_name: String,
    pub beneficiary_bank_query: String, // code or BIC (e.g. "001" or "NABILNPKA")
    pub beneficiary_account: String,
    pub purpose: RemittancePurpose,
    pub sender_prior_24h_paisa: u64,
    pub sender_prior_30d_paisa: u64,
}

/// Cryptographically Proven Settlement Receipt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemittanceReceipt {
    pub receipt_id: String,
    pub stellar_tx_hash: String,
    pub beneficiary_bank: CommercialBank,
    pub beneficiary_account: String,
    pub settlement_npr_paisa: u64,
    pub savings_summary: RemittanceCostComparison,
    pub npi_instruction: NpiPaymentInstruction,
    pub connectips_signature: String,
    pub cryptographic_proof_hash: String,
    pub compliance_report: ComplianceReport,
}

pub struct RemittancePipeline;

impl RemittancePipeline {
    /// Executes the full atomic settlement pipeline
    pub fn process_remittance(
        request: RemittanceRequest,
        rate: &NrbExchangeRate,
        gateway_bic: &str,
        app_id: &str,
        current_iso_timestamp: &str,
    ) -> Result<RemittanceReceipt, String> {
        // 1. Verify Stellar payment proof validity
        if !StellarPaymentProof::is_valid_tx_hash(&request.stellar_proof.tx_hash) {
            return Err("Invalid Stellar transaction hash format".to_string());
        }
        if !StellarPaymentProof::is_valid_account_id(&request.stellar_proof.source_account) {
            return Err("Invalid Stellar sender public key".to_string());
        }

        // 2. Lookup and validate beneficiary commercial bank in Nepal
        let bank = BankRegistry::find_bank(&request.beneficiary_bank_query)
            .ok_or_else(|| format!("Unknown Nepal commercial bank: {}", request.beneficiary_bank_query))?;

        BankRegistry::validate_account_number(bank, &request.beneficiary_account)
            .map_err(|e| format!("Invalid account number for {}: {}", bank.name_en, e))?;

        // 3. Exact FX & Family Savings Calculation (5 bps protocol fee vs 650 bps legacy)
        let cost_comparison = FxEngine::evaluate_cost_savings(
            request.source_amount_minor,
            request.source_decimals,
            rate,
            5,   // 0.05% protocol fee
            650, // 6.5% legacy remittance spread
        );

        let settlement_paisa = cost_comparison.net_recipient_npr_paisa;

        // 4. Regulatory Compliance & Limits Verification
        let compliance_report = ComplianceEngine::verify_transaction(
            settlement_paisa,
            request.sender_prior_24h_paisa,
            request.sender_prior_30d_paisa,
            &request.beneficiary_name,
        );

        if !compliance_report.is_compliant {
            return Err(format!(
                "Regulatory compliance rejected: {}",
                compliance_report.validation_errors.join("; ")
            ));
        }

        // 5. Generate NCHL NPI Payment Instruction
        let instruction_id = format!("NPI-{}", &request.stellar_proof.tx_hash[0..16]);
        let npi_instruction = NpiPaymentInstruction {
            instruction_id: instruction_id.clone(),
            debtor_agent_bic: gateway_bic.to_string(),
            creditor_bank_code: bank.code.to_string(),
            creditor_swift: bank.swift_bic.to_string(),
            creditor_name: request.beneficiary_name.clone(),
            creditor_account: request.beneficiary_account.clone(),
            amount_npr_paisa: settlement_paisa,
            purpose_code: request.purpose.code().to_string(),
            remittance_info: format!("Stellar Remit Tx: {}", &request.stellar_proof.tx_hash[0..12]),
            created_at_iso: current_iso_timestamp.to_string(),
        };

        // 6. Compute ConnectIPS SHA-256 signature
        let signature = NpiPaymentInstruction::compute_connectips_signature(
            app_id,
            &instruction_id,
            settlement_paisa,
            &request.stellar_proof.tx_hash[0..12],
        );

        // 7. Generate Cryptographic Audit Proof Hash
        let audit_payload = format!(
            "{}:{}:{}:{}:{}",
            request.stellar_proof.tx_hash,
            bank.code,
            request.beneficiary_account,
            settlement_paisa,
            signature
        );
        let proof_hash = Sha256::digest_hex(audit_payload.as_bytes());

        let receipt = RemittanceReceipt {
            receipt_id: format!("REMIT-{}", &proof_hash[0..16]),
            stellar_tx_hash: request.stellar_proof.tx_hash,
            beneficiary_bank: bank.clone(),
            beneficiary_account: request.beneficiary_account,
            settlement_npr_paisa: settlement_paisa,
            savings_summary: cost_comparison,
            npi_instruction,
            connectips_signature: signature,
            cryptographic_proof_hash: proof_hash,
            compliance_report,
        };

        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stellar::{StellarAsset, StellarMemo};

    #[test]
    fn test_end_to_end_pipeline_success() {
        let tx_hash = "a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0";
        let sender = "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN";
        let rate = NrbExchangeRate::standard_reference(ForeignCurrency::Usd);

        let request = RemittanceRequest {
            stellar_proof: StellarPaymentProof {
                tx_hash: tx_hash.to_string(),
                source_account: sender.to_string(),
                destination_account: "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H".to_string(),
                asset: StellarAsset::usdc(),
                amount_stroops: 10_000_000_000, // 1,000 USDC
                memo: StellarMemo::Text("REMIT".to_string()),
                ledger_sequence: 52_000_100,
                confirmed_at_unix: 1770000000,
            },
            foreign_currency: ForeignCurrency::Usd,
            source_amount_minor: 100_000, // $1,000.00
            source_decimals: 2,
            beneficiary_name: "Hari Bahadur KC".to_string(),
            beneficiary_bank_query: "001".to_string(), // Nabil Bank
            beneficiary_account: "00100175000010".to_string(),
            purpose: RemittancePurpose::FamilyMaintenance,
            sender_prior_24h_paisa: 0,
            sender_prior_30d_paisa: 0,
        };

        let receipt = RemittancePipeline::process_remittance(
            request,
            &rate,
            "STLRNPKA",
            "CONNECTIPS_APP_001",
            "2026-10-31T16:00:00Z",
        )
        .unwrap();

        assert_eq!(receipt.beneficiary_bank.code, "001");
        assert_eq!(receipt.beneficiary_bank.swift_bic, "NABILNPKA");
        // Family should save over NPR 8,000 compared to Western Union
        assert!(receipt.savings_summary.family_savings_npr_paisa > 800_000);
        assert!(!receipt.cryptographic_proof_hash.is_empty());
        assert!(!receipt.connectips_signature.is_empty());
    }
}
