# nepal-remit-core 🇳🇵⚡

[![CI](https://github.com/Ranjeet2063/nepal-remit-core/actions/workflows/ci.yml/badge.svg)](https://github.com/Ranjeet2063/nepal-remit-core/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-%3E%3D22.0.0-green.svg)](https://nodejs.org/)

**The world's first open-source, non-custodial cross-border settlement gateway bridging Stellar / Soroban to Nepal Clearing House Limited (NCHL / ConnectIPS / NPI) payment batch format.**

---

## 🌍 The National Pain & Economic Mission

- **National Lifeblood**: Remittances bring over **$11 Billion annually (~26% of Nepal's GDP)** into the country, sent by millions of hardworking Nepali migrant workers in the Gulf (UAE, Qatar, Saudi Arabia), Malaysia, Europe, the UK, USA, Australia, and Japan.
- **Predatory Middleman Fees**: Traditional legacy remittance corridors (Western Union, IME, MoneyGram, and high bank wire spreads) impose **5% to 8% in fees**, draining over **$600 Million (रु. 8,000+ Crore NPR)** out of the pockets of Nepali families every single year.
- **The Breakthrough**: `nepal-remit-core` slashes remittance fees down to **<0.1% (5 basis points)** by combining sub-3 second settlement on Stellar/Soroban with direct, instantaneous bank clearing through the NCHL National Payment Interface (NPI) and ConnectIPS.

---

## ⚡ Key Features

1. **Stellar & Soroban Non-Custodial Bridge**:
   - Parses and validates Stellar transactions, StrKey public keys (`G...`), 64-character transaction hashes, and payment memos.
   - Native support for Circle USDC (`GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN`), Circle EURC, and native XLM.
   - Formally verified Hash-Time Locked Contract (HTLC) state machine (`Initiated` → `Funded` → `Claimed` / `Refunded`) ensuring zero trapped or double-spent funds.

2. **Ramanujan Exact Integer FX & Savings Engine**:
   - Zero floating-point rounding errors: computes exact foreign exchange conversion at the single **paisa** level (1 NPR = 100 paisa) compliant with the Nepal Rastra Bank (NRB) Foreign Exchange Management Act.
   - Pre-configured reference curves for key migrant worker corridors: USD, AED (Dubai/Abu Dhabi), QAR (Qatar), SAR (Saudi Arabia), MYR (Malaysia), EUR, GBP, AUD, and JPY.
   - Built-in Family Savings Calculator comparing legacy 6.5% fees against the protocol's 0.05% corridor.

3. **Complete Nepal Commercial Banks Directory & NCHL NPI Generator**:
   - Canonical directory of all **20 Class "A" Commercial Banks of Nepal** (Nabil, Global IME, NIC Asia, Rastriya Banijya Bank, Nepal Investment Mega Bank, Standard Chartered, Himalayan Bank, etc.) with official SWIFT BICs, 3-digit NCHL routing codes, and account length validation.
   - Generates standard **ISO 20022 `pain.001.001.03`** Customer Credit Transfer Initiation XML messages for NCHL clearing batches.
   - Computes canonical **ConnectIPS digital signature hashes** (`SHA-256("appId,txnId,txnAmt,refId")`).

4. **Nepal Rastra Bank (NRB) Remittance Compliance Engine**:
   - Enforces statutory NRB Remittance Bylaws:
     - Minimum transfer threshold: NPR 100.00 (10,000 paisa)
     - Maximum single transaction limit: NPR 1,000,000.00 (100,000,000 paisa)
     - Daily rolling limit: NPR 1,500,000.00 (150,000,000 paisa)
     - Monthly rolling limit: NPR 5,000,000.00 (500,000,000 paisa)
     - Automatic Enhanced Due Diligence (EDD) flagging for transactions ≥ NPR 500,000.00
   - Purpose of Remittance codes (`FAMM`, `SAVI`, `EDUC`, `MEDI`, `DEBT`, `EMRG`) with bilingual English and Nepali descriptions.

5. **Dual High-Performance Architecture**:
   - **Rust Crate**: Pure zero-dependency crate with built-in FIPS 180-4 SHA-256 implementation, compiling in under 1 second.
   - **TypeScript Engine**: Native ESM package (`@ranjeet2063/nepal-remit-core`) tested with Node 22 native type stripping.

---

## 🏦 Nepal Commercial Banks Directory (20 Class "A" Banks)

| Bank Code | SWIFT BIC | Bank Name (English) | Bank Name (Devanagari) |
|---|---|---|---|
| `001` | `NABILNPKA` | Nabil Bank Limited | नबिल बैंक लिमिटेड |
| `002` | `NIBLNPKT` | Nepal Investment Mega Bank Limited | नेपाल इन्भेष्टमेण्ट मेगा बैंक लिमिटेड |
| `003` | `SCBLNPKA` | Standard Chartered Bank Nepal Limited | स्ट्यान्डर्ड चार्टर्ड बैंक नेपाल लिमिटेड |
| `004` | `HIMANPKA` | Himalayan Bank Limited | हिमालयन बैंक लिमिटेड |
| `005` | `NSBINPKT` | Nepal SBI Bank Limited | नेपाल एसबिआई बैंक लिमिटेड |
| `006` | `EVBLNPKA` | Everest Bank Limited | एभरेष्ट बैंक लिमिटेड |
| `007` | `PRBUNPKA` | Prabhu Bank Limited | प्रभु बैंक लिमिटेड |
| `008` | `LXMINPKA` | Laxmi Sunrise Bank Limited | लक्ष्मी सनराइज बैंक लिमिटेड |
| `009` | `GLBINPKA` | Global IME Bank Limited | ग्लोबल आइएमई बैंक लिमिटेड |
| `010` | `CZBINPKA` | Citizens Bank International Limited | सिटिजन्स बैंक इन्टरनेसनल लिमिटेड |
| `011` | `PCBLNPKA` | Prime Commercial Bank Limited | प्राइम कमर्सियल बैंक लिमिटेड |
| `012` | `SNBLNPKA` | Sunrise Bank Limited | सनराइज बैंक लिमिटेड |
| `013` | `KMBLNPKA` | Kumari Bank Limited | कुमारी बैंक लिमिटेड |
| `014` | `SIDDNPKA` | Siddhartha Bank Limited | सिद्धार्थ बैंक लिमिटेड |
| `015` | `ADBLNPKA` | Agriculture Development Bank Limited | कृषि विकास बैंक लिमिटेड |
| `016` | `NICAINPKA` | NIC Asia Bank Limited | एनआईसी एशिया बैंक लिमिटेड |
| `017` | `MBLINPKT` | Machhapuchchhre Bank Limited | माछापुच्छ्रे बैंक लिमिटेड |
| `018` | `SNMANPKA` | Sanima Bank Limited | सानिमा बैंक लिमिटेड |
| `019` | `RBBANPKA` | Rastriya Banijya Bank Limited | राष्ट्रिय वाणिज्य बैंक लिमिटेड |
| `020` | `NEBLNPKA` | Nepal Bank Limited | नेपाल बैंक लिमिटेड |

---

## 🚀 Quickstart

### Rust

Add to your `Cargo.toml`:
```toml
[dependencies]
nepal-remit-core = { git = "https://github.com/Ranjeet2063/nepal-remit-core" }
```

```rust
use nepal_remit_core::{
    BankRegistry, ForeignCurrency, FxEngine, NrbExchangeRate,
    RemittancePipeline, RemittanceRequest, RemittancePurpose,
    StellarAsset, StellarMemo, StellarPaymentProof,
};

fn main() {
    // 1. Reference NRB exchange rate
    let rate = NrbExchangeRate::standard_reference(ForeignCurrency::Usd);

    // 2. Prepare verified Stellar inbound remittance
    let request = RemittanceRequest {
        stellar_proof: StellarPaymentProof {
            tx_hash: "a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0".into(),
            source_account: "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN".into(),
            destination_account: "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H".into(),
            asset: StellarAsset::usdc(),
            amount_stroops: 10_000_000_000, // 1,000 USDC
            memo: StellarMemo::Text("REMIT".into()),
            ledger_sequence: 52_000_100,
            confirmed_at_unix: 1770000000,
        },
        foreign_currency: ForeignCurrency::Usd,
        source_amount_minor: 100_000, // $1,000.00
        source_decimals: 2,
        beneficiary_name: "Hari Bahadur KC".into(),
        beneficiary_bank_query: "001".into(), // Nabil Bank
        beneficiary_account: "00100175000010".into(),
        purpose: RemittancePurpose::FamilyMaintenance,
        sender_prior_24h_paisa: 0,
        sender_prior_30d_paisa: 0,
    };

    // 3. Process atomic settlement pipeline
    let receipt = RemittancePipeline::process_remittance(
        request,
        &rate,
        "STLRNPKA",
        "CONNECTIPS_APP_001",
        "2026-10-31T16:00:00Z",
    ).expect("Remittance settlement succeeded");

    println!("Receipt ID: {}", receipt.receipt_id);
    println!("Settlement Amount: {} paisa ({} NPR)", receipt.settlement_npr_paisa, receipt.settlement_npr_paisa / 100);
    println!("Family Saved: {} paisa vs Western Union", receipt.savings_summary.family_savings_npr_paisa);
    println!("ConnectIPS Signature: {}", receipt.connectips_signature);
}
```

### TypeScript / Node.js

```typescript
import {
  STANDARD_NRB_RATES,
  RemittancePipeline,
  CIRCLE_USDC,
} from "@ranjeet2063/nepal-remit-core";

const rate = STANDARD_NRB_RATES.AED; // 2,000 AED sent from Dubai

const receipt = RemittancePipeline.processRemittance(
  {
    stellarTxHash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    sourceAccount: "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
    destinationAccount: "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H",
    asset: CIRCLE_USDC,
    amountStroops: 2_000_000_000n,
    foreignCurrency: "AED",
    sourceAmountMinor: 200_000n,
    sourceDecimals: 2,
    beneficiaryName: "Gita Devi Thapa",
    beneficiaryBankQuery: "GLBINPKA", // Global IME Bank
    beneficiaryAccount: "00900175000010",
    purposeCode: "FAMM",
    senderPrior24hPaisa: 0n,
    senderPrior30dPaisa: 0n,
  },
  rate,
  "STLRNPKA",
  "CONNECTIPS_APP_001",
  "2026-10-31T16:00:00Z",
);

console.log("Settlement:", receipt.settlementNprPaisa, "paisa");
console.log("ConnectIPS Signature:", receipt.connectipsSignature);
```

---

## 🧪 Verification & Formal Testing

Run Rust tests:
```bash
cargo test --release
```

Run TypeScript tests:
```bash
cd ts && node --test --experimental-strip-types test/index.test.ts
```

---

## 📄 License

MIT OR Apache-2.0. Built with pride for Nepal by [Ranjeet Kumar Gupta (@Ranjeet2063)](https://github.com/Ranjeet2063).
