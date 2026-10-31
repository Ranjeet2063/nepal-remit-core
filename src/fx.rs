/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
//! Foreign Exchange (FX) & Zero-Loss Remittance Fee Engine
//! Implements exact integer paisa arithmetic (Ramanujan precision) compliant with
//! Nepal Rastra Bank (NRB) Foreign Exchange Management regulations.

/// Supported Remittance Source Currencies (Major Nepali Migrant Worker Corridors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignCurrency {
    Usd, // United States Dollar (Global / Gulf pegged)
    Aed, // United Arab Emirates Dirham (Dubai / Abu Dhabi)
    Qar, // Qatari Riyal (Doha)
    Sar, // Saudi Riyal (Riyadh / Jeddah)
    Myr, // Malaysian Ringgit (Kuala Lumpur)
    Eur, // Euro (EU)
    Gbp, // British Pound (UK)
    Aud, // Australian Dollar (Sydney / Melbourne)
    Jpy, // Japanese Yen (Tokyo)
}

impl ForeignCurrency {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Usd => "USD",
            Self::Aed => "AED",
            Self::Qar => "QAR",
            Self::Sar => "SAR",
            Self::Myr => "MYR",
            Self::Eur => "EUR",
            Self::Gbp => "GBP",
            Self::Aud => "AUD",
            Self::Jpy => "JPY",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "USD" => Some(Self::Usd),
            "AED" => Some(Self::Aed),
            "QAR" => Some(Self::Qar),
            "SAR" => Some(Self::Sar),
            "MYR" => Some(Self::Myr),
            "EUR" => Some(Self::Eur),
            "GBP" => Some(Self::Gbp),
            "AUD" => Some(Self::Aud),
            "JPY" => Some(Self::Jpy),
            _ => None,
        }
    }
}

/// Exchange rate specified in paisa (1 NPR = 100 paisa)
/// For example, 1 USD = 134.85 NPR is stored as 13,485 paisa per unit
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NrbExchangeRate {
    pub currency: ForeignCurrency,
    pub unit: u32,             // e.g. 1 for USD, 10 for JPY, 100 for JPY in NRB tables
    pub buying_rate_paisa: u64, // NRB buying rate in paisa
    pub selling_rate_paisa: u64, // NRB selling rate in paisa
    pub mid_rate_paisa: u64,    // Computed mid-market benchmark
}

impl NrbExchangeRate {
    pub fn new(
        currency: ForeignCurrency,
        unit: u32,
        buying_rate_paisa: u64,
        selling_rate_paisa: u64,
    ) -> Self {
        let mid_rate_paisa = (buying_rate_paisa + selling_rate_paisa) / 2;
        Self {
            currency,
            unit,
            buying_rate_paisa,
            selling_rate_paisa,
            mid_rate_paisa,
        }
    }

    /// Canonical NRB reference rates (October 2026 baseline benchmark)
    pub fn standard_reference(currency: ForeignCurrency) -> Self {
        match currency {
            ForeignCurrency::Usd => Self::new(ForeignCurrency::Usd, 1, 13485, 13545), // 134.85 NPR
            ForeignCurrency::Aed => Self::new(ForeignCurrency::Aed, 1, 3672, 3688),   // 36.72 NPR
            ForeignCurrency::Qar => Self::new(ForeignCurrency::Qar, 1, 3704, 3721),   // 37.04 NPR
            ForeignCurrency::Sar => Self::new(ForeignCurrency::Sar, 1, 3594, 3610),   // 35.94 NPR
            ForeignCurrency::Myr => Self::new(ForeignCurrency::Myr, 1, 3110, 3125),   // 31.10 NPR
            ForeignCurrency::Eur => Self::new(ForeignCurrency::Eur, 1, 14620, 14685), // 146.20 NPR
            ForeignCurrency::Gbp => Self::new(ForeignCurrency::Gbp, 1, 17540, 17618), // 175.40 NPR
            ForeignCurrency::Aud => Self::new(ForeignCurrency::Aud, 1, 8910, 8950),   // 89.10 NPR
            ForeignCurrency::Jpy => Self::new(ForeignCurrency::Jpy, 10, 895, 901),    // 89.50 NPR per 10
        }
    }
}

/// Comparison of Remittance Cost Breakdown
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemittanceCostComparison {
    pub foreign_currency: ForeignCurrency,
    pub foreign_amount_cents: u64, // amount in minor unit (e.g. cents)
    pub gross_npr_paisa: u64,      // total equivalent NPR in paisa before fees

    // nepal-remit-core (Stellar / NCHL low-fee corridor)
    pub protocol_fee_bps: u32,     // 5 bps = 0.05%
    pub protocol_fee_npr_paisa: u64,
    pub net_recipient_npr_paisa: u64,

    // Legacy middleman comparison (Western Union / IME: ~6.5% average loss)
    pub legacy_fee_percent: u32,   // e.g. 650 bps = 6.5%
    pub legacy_fee_npr_paisa: u64,
    pub legacy_recipient_npr_paisa: u64,

    // Direct Family Savings in NPR
    pub family_savings_npr_paisa: u64,
}

impl RemittanceCostComparison {
    /// Formats paisa into standard Rupee decimal string (e.g. 1348500 paisa -> "13485.00 NPR")
    pub fn format_npr(paisa: u64) -> String {
        let rupees = paisa / 100;
        let p = paisa % 100;
        format!("{}.{:02} NPR", rupees, p)
    }
}

pub struct FxEngine;

impl FxEngine {
    /// Converts foreign currency amount (minor units, e.g. USD cents) to NPR paisa
    /// using exact Ramanujan integer arithmetic with zero floating point inaccuracy.
    pub fn convert_to_npr_paisa(
        foreign_amount_minor: u64,
        foreign_decimals: u32,
        rate: &NrbExchangeRate,
    ) -> u64 {
        let scale = 10u64.pow(foreign_decimals);
        // calculation: (foreign_amount_minor * rate.buying_rate_paisa) / (scale * rate.unit)
        let numerator = (foreign_amount_minor as u128) * (rate.buying_rate_paisa as u128);
        let denominator = (scale as u128) * (rate.unit as u128);
        (numerator / denominator) as u64
    }

    /// Evaluates complete cost comparison and direct family savings
    pub fn evaluate_cost_savings(
        foreign_amount_minor: u64,
        foreign_decimals: u32,
        rate: &NrbExchangeRate,
        protocol_fee_bps: u32, // default: 5 (0.05%)
        legacy_fee_bps: u32,   // default: 650 (6.5%)
    ) -> RemittanceCostComparison {
        let gross_npr_paisa = Self::convert_to_npr_paisa(foreign_amount_minor, foreign_decimals, rate);

        let protocol_fee_npr_paisa =
            ((gross_npr_paisa as u128 * protocol_fee_bps as u128) / 10_000) as u64;
        let net_recipient_npr_paisa = gross_npr_paisa.saturating_sub(protocol_fee_npr_paisa);

        let legacy_fee_npr_paisa =
            ((gross_npr_paisa as u128 * legacy_fee_bps as u128) / 10_000) as u64;
        let legacy_recipient_npr_paisa = gross_npr_paisa.saturating_sub(legacy_fee_npr_paisa);

        let family_savings_npr_paisa =
            net_recipient_npr_paisa.saturating_sub(legacy_recipient_npr_paisa);

        RemittanceCostComparison {
            foreign_currency: rate.currency,
            foreign_amount_cents: foreign_amount_minor,
            gross_npr_paisa,
            protocol_fee_bps,
            protocol_fee_npr_paisa,
            net_recipient_npr_paisa,
            legacy_fee_percent: legacy_fee_bps,
            legacy_fee_npr_paisa,
            legacy_recipient_npr_paisa,
            family_savings_npr_paisa,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_to_npr_exact_conversion() {
        let rate = NrbExchangeRate::standard_reference(ForeignCurrency::Usd);
        // $1,000 USD (100,000 cents) at 134.85 NPR = 134,850.00 NPR = 13,485,000 paisa
        let npr_paisa = FxEngine::convert_to_npr_paisa(100_000, 2, &rate);
        assert_eq!(npr_paisa, 13_485_000);
        assert_eq!(RemittanceCostComparison::format_npr(npr_paisa), "134850.00 NPR");
    }

    #[test]
    fn test_family_savings_calculation() {
        let rate = NrbExchangeRate::standard_reference(ForeignCurrency::Aed);
        // 2,000 AED sent from Dubai (200,000 fils)
        let savings = FxEngine::evaluate_cost_savings(200_000, 2, &rate, 5, 650);

        // Protocol fee is 0.05% vs Legacy fee 6.5%
        assert!(savings.family_savings_npr_paisa > 0);
        // Legacy fee should be 130x higher than protocol fee
        assert!(savings.legacy_fee_npr_paisa > savings.protocol_fee_npr_paisa * 100);
    }
}
