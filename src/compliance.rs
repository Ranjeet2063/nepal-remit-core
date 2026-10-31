/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
//! Nepal Rastra Bank (NRB) Remittance Bylaws & Anti-Money Laundering (AML) Compliance Engine
//! Enforces regulatory transaction thresholds, statutory limits, and purpose-of-remittance invariants.

/// Official NRB Remittance Purpose Classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemittancePurpose {
    FamilyMaintenance, // घर खर्च / परिवार पालन
    SavingsInvestment, // बचत तथा लगानी
    Education,         // शिक्षा खर्च
    MedicalTreatment,  // स्वास्थ्य उपचार
    DebtRepayment,     // ऋण भुक्तानी
    EmergencyAid,      // आकस्मिक सहयोग
}

impl RemittancePurpose {
    pub fn code(&self) -> &'static str {
        match self {
            Self::FamilyMaintenance => "FAMM",
            Self::SavingsInvestment => "SAVI",
            Self::Education => "EDUC",
            Self::MedicalTreatment => "MEDI",
            Self::DebtRepayment => "DEBT",
            Self::EmergencyAid => "EMRG",
        }
    }

    pub fn description_en(&self) -> &'static str {
        match self {
            Self::FamilyMaintenance => "Family Maintenance & Living Expenses",
            Self::SavingsInvestment => "Personal Savings & Capital Investment",
            Self::Education => "Educational & Tuition Expenses",
            Self::MedicalTreatment => "Medical Treatment & Healthcare",
            Self::DebtRepayment => "Loan & Debt Service Repayment",
            Self::EmergencyAid => "Emergency Humanitarian Aid",
        }
    }

    pub fn description_ne(&self) -> &'static str {
        match self {
            Self::FamilyMaintenance => "घर खर्च / परिवार पालन",
            Self::SavingsInvestment => "बचत तथा लगानी",
            Self::Education => "शिक्षा खर्च",
            Self::MedicalTreatment => "स्वास्थ्य उपचार",
            Self::DebtRepayment => "ऋण भुक्तानी",
            Self::EmergencyAid => "आकस्मिक सहयोग",
        }
    }
}

/// Regulatory Invariants enforced by Nepal Rastra Bank (NRB) Remittance Bylaws
pub struct ComplianceLimits;

impl ComplianceLimits {
    /// Minimum transfer threshold: NPR 100.00 (10,000 paisa)
    pub const MIN_TRANSACTION_PAISA: u64 = 10_000;

    /// Maximum single transaction limit: NPR 1,000,000.00 (100,000,000 paisa)
    pub const MAX_SINGLE_TXN_PAISA: u64 = 100_000_000;

    /// Maximum daily rolling limit per sender: NPR 1,500,000.00 (150,000,000 paisa)
    pub const MAX_DAILY_SENDER_PAISA: u64 = 150_000_000;

    /// Maximum monthly rolling limit per sender: NPR 5,000,000.00 (500,000,000 paisa)
    pub const MAX_MONTHLY_SENDER_PAISA: u64 = 500_000_000;
}

/// Remittance Compliance Verification Report
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplianceReport {
    pub is_compliant: bool,
    pub validation_errors: Vec<String>,
    pub requires_enhanced_due_diligence: bool,
}

pub struct ComplianceEngine;

impl ComplianceEngine {
    /// Evaluates if an intended remittance satisfies all NRB statutory limits
    pub fn verify_transaction(
        amount_paisa: u64,
        prior_24h_amount_paisa: u64,
        prior_30d_amount_paisa: u64,
        beneficiary_name: &str,
    ) -> ComplianceReport {
        let mut errors = Vec::new();

        // 1. Min Threshold
        if amount_paisa < ComplianceLimits::MIN_TRANSACTION_PAISA {
            errors.push(format!(
                "Amount {} paisa is below NRB minimum transfer threshold of {} paisa (NPR 100.00)",
                amount_paisa,
                ComplianceLimits::MIN_TRANSACTION_PAISA
            ));
        }

        // 2. Max Single Transaction Limit
        if amount_paisa > ComplianceLimits::MAX_SINGLE_TXN_PAISA {
            errors.push(format!(
                "Amount {} paisa exceeds NRB maximum single transaction limit of NPR 1,000,000.00",
                amount_paisa
            ));
        }

        // 3. Daily Aggregate Limit
        let total_daily = prior_24h_amount_paisa.saturating_add(amount_paisa);
        if total_daily > ComplianceLimits::MAX_DAILY_SENDER_PAISA {
            errors.push(format!(
                "Daily cumulative volume {} paisa exceeds NRB 24-hour limit of NPR 1,500,000.00",
                total_daily
            ));
        }

        // 4. Monthly Aggregate Limit
        let total_monthly = prior_30d_amount_paisa.saturating_add(amount_paisa);
        if total_monthly > ComplianceLimits::MAX_MONTHLY_SENDER_PAISA {
            errors.push(format!(
                "Monthly cumulative volume {} paisa exceeds NRB 30-day limit of NPR 5,000,000.00",
                total_monthly
            ));
        }

        // 5. Basic Beneficiary Validation
        if beneficiary_name.trim().len() < 3 {
            errors.push("Beneficiary legal name must be at least 3 characters".to_string());
        }

        // 6. Enhanced Due Diligence (EDD) threshold check (>= NPR 500,000)
        let requires_edd = amount_paisa >= 50_000_000;

        ComplianceReport {
            is_compliant: errors.is_empty(),
            validation_errors: errors,
            requires_enhanced_due_diligence: requires_edd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_compliance() {
        let report = ComplianceEngine::verify_transaction(
            13_485_000, // NPR 134,850.00
            0,
            0,
            "Ram Bahadur Thapa",
        );
        assert!(report.is_compliant);
        assert!(report.validation_errors.is_empty());
        assert!(!report.requires_enhanced_due_diligence);
    }

    #[test]
    fn test_exceeding_single_limit() {
        let report = ComplianceEngine::verify_transaction(
            120_000_000, // NPR 1,200,000.00 (Exceeds 1M limit)
            0,
            0,
            "Sita Kumari Sharma",
        );
        assert!(!report.is_compliant);
        assert_eq!(report.validation_errors.len(), 1);
    }

    #[test]
    fn test_enhanced_due_diligence_flag() {
        let report = ComplianceEngine::verify_transaction(
            60_000_000, // NPR 600,000.00 (Under 1M limit but >= 500k EDD threshold)
            0,
            0,
            "Hari Prasad Koirala",
        );
        assert!(report.is_compliant);
        assert!(report.requires_enhanced_due_diligence);
    }
}
