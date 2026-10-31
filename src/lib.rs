/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
//! # nepal-remit-core
//!
//! Non-custodial cross-border settlement gateway bridging Stellar / Soroban (USDC, EURC, XLM)
//! to Nepal Clearing House Limited (NCHL / ConnectIPS / NPI) payment batch format.
//!
//! Compliant with Nepal Rastra Bank (NRB) Foreign Exchange Management Act and Remittance Bylaws.
//! Eliminates predatory 5% to 8% middleman fees (Western Union, IME) with a <0.1% settlement corridor.

pub mod compliance;
pub mod crypto;
pub mod fx;
pub mod nchl;
pub mod pipeline;
pub mod stellar;

// Clean top-level re-exports
pub use compliance::{ComplianceEngine, ComplianceLimits, ComplianceReport, RemittancePurpose};
pub use crypto::Sha256;
pub use fx::{ForeignCurrency, FxEngine, NrbExchangeRate, RemittanceCostComparison};
pub use nchl::{BankRegistry, CommercialBank, NpiPaymentInstruction, COMMERCIAL_BANKS};
pub use pipeline::{RemittancePipeline, RemittanceReceipt, RemittanceRequest};
pub use stellar::{
    EscrowStatus, RemittanceEscrow, StellarAsset, StellarMemo, StellarPaymentProof,
};
