/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
//! NCHL National Payment Interface (NPI) & ConnectIPS Settlement Engine
//! Implements canonical Nepal Commercial Banks directory, ISO 20022 pain.001 message generation,
//! and ConnectIPS SHA-256 transaction signature verification.

use crate::crypto::Sha256;

/// Class "A" Commercial Bank of Nepal
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommercialBank {
    pub code: &'static str,       // 3-digit NCHL routing code
    pub swift_bic: &'static str,  // SWIFT BIC code
    pub name_en: &'static str,    // Official English Name
    pub name_ne: &'static str,    // Official Devanagari Name
    pub min_acc_len: usize,       // Min account number length
    pub max_acc_len: usize,       // Max account number length
}

/// The complete canonical directory of all 20 Class "A" Commercial Banks of Nepal
pub const COMMERCIAL_BANKS: [CommercialBank; 20] = [
    CommercialBank {
        code: "001",
        swift_bic: "NABILNPKA",
        name_en: "Nabil Bank Limited",
        name_ne: "नबिल बैंक लिमिटेड",
        min_acc_len: 13,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "002",
        swift_bic: "NIBLNPKT",
        name_en: "Nepal Investment Mega Bank Limited",
        name_ne: "नेपाल इन्भेष्टमेण्ट मेगा बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "003",
        swift_bic: "SCBLNPKA",
        name_en: "Standard Chartered Bank Nepal Limited",
        name_ne: "स्ट्यान्डर्ड चार्टर्ड बैंक नेपाल लिमिटेड",
        min_acc_len: 13,
        max_acc_len: 15,
    },
    CommercialBank {
        code: "004",
        swift_bic: "HIMANPKA",
        name_en: "Himalayan Bank Limited",
        name_ne: "हिमालयन बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "005",
        swift_bic: "NSBINPKT",
        name_en: "Nepal SBI Bank Limited",
        name_ne: "नेपाल एसबिआई बैंक लिमिटेड",
        min_acc_len: 16,
        max_acc_len: 18,
    },
    CommercialBank {
        code: "006",
        swift_bic: "EVBLNPKA",
        name_en: "Everest Bank Limited",
        name_ne: "एभरेष्ट बैंक लिमिटेड",
        min_acc_len: 12,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "007",
        swift_bic: "PRBUNPKA",
        name_en: "Prabhu Bank Limited",
        name_ne: "प्रभु बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "008",
        swift_bic: "LXMINPKA",
        name_en: "Laxmi Sunrise Bank Limited",
        name_ne: "लक्ष्मी सनराइज बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "009",
        swift_bic: "GLBINPKA",
        name_en: "Global IME Bank Limited",
        name_ne: "ग्लोबल आइएमई बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 17,
    },
    CommercialBank {
        code: "010",
        swift_bic: "CZBINPKA",
        name_en: "Citizens Bank International Limited",
        name_ne: "सिटिजन्स बैंक इन्टरनेसनल लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "011",
        swift_bic: "PCBLNPKA",
        name_en: "Prime Commercial Bank Limited",
        name_ne: "प्राइम कमर्सियल बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "012",
        swift_bic: "SNBLNPKA",
        name_en: "Sunrise Bank Limited",
        name_ne: "सनराइज बैंक लिमिटेड",
        min_acc_len: 13,
        max_acc_len: 15,
    },
    CommercialBank {
        code: "013",
        swift_bic: "KMBLNPKA",
        name_en: "Kumari Bank Limited",
        name_ne: "कुमारी बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "014",
        swift_bic: "SIDDNPKA",
        name_en: "Siddhartha Bank Limited",
        name_ne: "सिद्धार्थ बैंक लिमिटेड",
        min_acc_len: 13,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "015",
        swift_bic: "ADBLNPKA",
        name_en: "Agriculture Development Bank Limited",
        name_ne: "कृषि विकास बैंक लिमिटेड",
        min_acc_len: 16,
        max_acc_len: 18,
    },
    CommercialBank {
        code: "016",
        swift_bic: "NICAINPKA",
        name_en: "NIC Asia Bank Limited",
        name_ne: "एनआईसी एशिया बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "017",
        swift_bic: "MBLINPKT",
        name_en: "Machhapuchchhre Bank Limited",
        name_ne: "माछापुच्छ्रे बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "018",
        swift_bic: "SNMANPKA",
        name_en: "Sanima Bank Limited",
        name_ne: "सानिमा बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "019",
        swift_bic: "RBBANPKA",
        name_en: "Rastriya Banijya Bank Limited",
        name_ne: "राष्ट्रिय वाणिज्य बैंक लिमिटेड",
        min_acc_len: 12,
        max_acc_len: 16,
    },
    CommercialBank {
        code: "020",
        swift_bic: "NEBLNPKA",
        name_en: "Nepal Bank Limited",
        name_ne: "नेपाल बैंक लिमिटेड",
        min_acc_len: 14,
        max_acc_len: 16,
    },
];

pub struct BankRegistry;

impl BankRegistry {
    /// Find a bank by 3-digit NCHL code or SWIFT BIC
    pub fn find_bank(query: &str) -> Option<&'static CommercialBank> {
        let q_upper = query.trim().to_uppercase();
        COMMERCIAL_BANKS.iter().find(|b| {
            b.code == query.trim() || b.swift_bic == q_upper || b.name_en.to_uppercase().contains(&q_upper)
        })
    }

    /// Validates an account number format for a specific bank
    pub fn validate_account_number(bank: &CommercialBank, account_number: &str) -> Result<(), &'static str> {
        let clean = account_number.trim();
        if !clean.chars().all(|c| c.is_ascii_digit()) {
            return Err("Bank account number must contain digits only");
        }
        if clean.len() < bank.min_acc_len || clean.len() > bank.max_acc_len {
            return Err("Bank account number length does not match bank specifications");
        }
        Ok(())
    }
}

/// NCHL National Payment Interface (NPI) Direct Credit Instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpiPaymentInstruction {
    pub instruction_id: String,     // Unique settlement UUID / ID
    pub debtor_agent_bic: String,   // Stellar Settlement Gateway BIC
    pub creditor_bank_code: String, // 3-digit NCHL bank code
    pub creditor_swift: String,     // Creditor bank SWIFT BIC
    pub creditor_name: String,      // Beneficiary legal full name in Nepal
    pub creditor_account: String,   // Beneficiary bank account number
    pub amount_npr_paisa: u64,      // Settlement amount in paisa
    pub purpose_code: String,       // Remittance purpose code (e.g. "REMT")
    pub remittance_info: String,    // Narration / Stellar Tx Hash reference
    pub created_at_iso: String,     // ISO 8601 timestamp
}

impl NpiPaymentInstruction {
    /// Computes ConnectIPS digital signature hash for the payload
    /// Signature format: SHA-256("appId={app_id},txnId={txn_id},txnAmt={amount_rupees},refId={ref_id}")
    pub fn compute_connectips_signature(
        app_id: &str,
        txn_id: &str,
        amount_paisa: u64,
        ref_id: &str,
    ) -> String {
        let amount_rupees = format!("{}.{:02}", amount_paisa / 100, amount_paisa % 100);
        let raw_payload = format!(
            "appId={},txnId={},txnAmt={},refId={}",
            app_id, txn_id, amount_rupees, ref_id
        );
        Sha256::digest_hex(raw_payload.as_bytes())
    }

    /// Generates canonical ISO 20022 pain.001.001.03 XML for NCHL clearing batch
    pub fn to_iso20022_pain001_xml(&self) -> String {
        let amount_rupees = format!("{}.{:02}", self.amount_npr_paisa / 100, self.amount_npr_paisa % 100);
        format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pain.001.001.03">
  <CstmrCdtTrfInitn>
    <GrpHdr>
      <MsgId>{}</MsgId>
      <CreDtTm>{}</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <CtrlSum>{}</CtrlSum>
      <InitgPty>
        <Nm>Stellar-Nepal-Remittance-Gateway</Nm>
      </InitgPty>
    </GrpHdr>
    <PmtInf>
      <PmtInfId>PMT-{}</PmtInfId>
      <PmtMtd>TRF</PmtMtd>
      <ReqdExctnDt>{}</ReqdExctnDt>
      <Dbtr>
        <Nm>NonCustodial Remittance Bridge</Nm>
      </Dbtr>
      <DbtrAgt>
        <FinInstnId>
          <BIC>{}</BIC>
        </FinInstnId>
      </DbtrAgt>
      <CdtTrfTxInf>
        <PmtId>
          <EndToEndId>{}</EndToEndId>
        </PmtId>
        <Amt>
          <InstdAmt Ccy="NPR">{}</InstdAmt>
        </Amt>
        <CdtrAgt>
          <FinInstnId>
            <BIC>{}</BIC>
            <ClrSysMmbId>
              <MmbId>{}</MmbId>
            </ClrSysMmbId>
          </FinInstnId>
        </CdtrAgt>
        <Cdtr>
          <Nm>{}</Nm>
        </Cdtr>
        <CdtrAcct>
          <Id>
            <Othr>
              <Id>{}</Id>
            </Othr>
          </Id>
        </CdtrAcct>
        <RmtInf>
          <Ustrd>{}</Ustrd>
        </RmtInf>
      </CdtTrfTxInf>
    </PmtInf>
  </CstmrCdtTrfInitn>
</Document>"#,
            self.instruction_id,
            self.created_at_iso,
            amount_rupees,
            self.instruction_id,
            &self.created_at_iso[0..10.min(self.created_at_iso.len())],
            self.debtor_agent_bic,
            self.instruction_id,
            amount_rupees,
            self.creditor_swift,
            self.creditor_bank_code,
            self.creditor_name,
            self.creditor_account,
            self.remittance_info
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commercial_banks_directory() {
        assert_eq!(COMMERCIAL_BANKS.len(), 20);

        // Lookup Nabil Bank
        let nabil = BankRegistry::find_bank("001").unwrap();
        assert_eq!(nabil.swift_bic, "NABILNPKA");
        assert_eq!(nabil.name_en, "Nabil Bank Limited");

        // Lookup Global IME
        let global_ime = BankRegistry::find_bank("GLBINPKA").unwrap();
        assert_eq!(global_ime.code, "009");
    }

    #[test]
    fn test_account_validation() {
        let nabil = BankRegistry::find_bank("001").unwrap();
        assert!(BankRegistry::validate_account_number(nabil, "00100175000010").is_ok());
        assert!(BankRegistry::validate_account_number(nabil, "123").is_err()); // Too short
        assert!(BankRegistry::validate_account_number(nabil, "00100175ABCD").is_err()); // Letters not allowed
    }

    #[test]
    fn test_iso20022_xml_generation() {
        let instr = NpiPaymentInstruction {
            instruction_id: "TXN-2026-0001".to_string(),
            debtor_agent_bic: "STLRNPKA".to_string(),
            creditor_bank_code: "001".to_string(),
            creditor_swift: "NABILNPKA".to_string(),
            creditor_name: "Ram Bahadur Thapa".to_string(),
            creditor_account: "00100175000010".to_string(),
            amount_npr_paisa: 13_485_000,
            purpose_code: "REMT".to_string(),
            remittance_info: "Stellar Tx a1b2c3d4".to_string(),
            created_at_iso: "2026-10-31T16:00:00Z".to_string(),
        };

        let xml = instr.to_iso20022_pain001_xml();
        assert!(xml.contains("<InstdAmt Ccy=\"NPR\">134850.00</InstdAmt>"));
        assert!(xml.contains("<BIC>NABILNPKA</BIC>"));
        assert!(xml.contains("<Nm>Ram Bahadur Thapa</Nm>"));
    }
}
