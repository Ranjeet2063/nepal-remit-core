/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  sha256Hex,
  isValidStellarAccountId,
  isValidStellarTxHash,
  RemittanceEscrow,
  CIRCLE_USDC,
  STANDARD_NRB_RATES,
  FxEngine,
  BankRegistry,
  COMMERCIAL_BANKS,
  toIso20022Pain001Xml,
  ComplianceEngine,
  COMPLIANCE_LIMITS,
  RemittancePipeline,
} from "../src/index.ts";
import type {
  RemittanceRequest,
  NpiPaymentInstruction,
} from "../src/index.ts";

describe("nepal-remit-core TypeScript Test Suite", () => {
  it("verifies SHA-256 cryptographic hashing", () => {
    assert.equal(
      sha256Hex("hello world"),
      "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
  });

  it("validates Stellar public keys and transaction hashes", () => {
    assert.equal(
      isValidStellarAccountId(
        "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
      ),
      true
    );
    assert.equal(isValidStellarAccountId("INVALID_KEY"), false);

    assert.equal(
      isValidStellarTxHash(
        "a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0"
      ),
      true
    );
  });

  it("verifies non-custodial HTLC escrow state machine", () => {
    const secret = "nchl_test_secret_preimage_2026";
    const hashLock = sha256Hex(secret);
    const timeout = 1800000000;

    const escrow = new RemittanceEscrow(
      "ESCROW-001",
      "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
      "NABILNPKA",
      "00100175000010",
      CIRCLE_USDC,
      100_000_000n, // 10 USDC
      hashLock,
      timeout
    );

    assert.equal(escrow.status.type, "Initiated");
    escrow.markFunded();
    assert.equal(escrow.status.type, "Funded");

    // Invalid secret throws
    assert.throws(() => escrow.claim("wrong_secret", timeout - 100));

    // Valid secret succeeds
    escrow.claim(secret, timeout - 100);
    assert.equal(escrow.status.type, "Claimed");
  });

  it("computes exact integer paisa FX conversion and family savings", () => {
    const rate = STANDARD_NRB_RATES.USD;
    // $1,000 USD (100,000 cents) at 134.85 NPR
    const nprPaisa = FxEngine.convertToNprPaisa(100_000n, 2, rate);
    assert.equal(nprPaisa, 13_485_000n); // 134,850.00 NPR
    assert.equal(FxEngine.formatNpr(nprPaisa), "134850.00 NPR");

    const savings = FxEngine.evaluateCostSavings(100_000n, 2, rate, 5, 650);
    assert.ok(savings.familySavingsNprPaisa > 800_000n); // Over NPR 8,000 saved vs Western Union
  });

  it("verifies 20 commercial banks directory and ISO 20022 XML generation", () => {
    assert.equal(COMMERCIAL_BANKS.length, 20);

    const nabil = BankRegistry.findBank("NABILNPKA");
    assert.ok(nabil);
    assert.equal(nabil?.code, "001");

    assert.doesNotThrow(() =>
      BankRegistry.validateAccountNumber(nabil!, "00100175000010")
    );
    assert.throws(() =>
      BankRegistry.validateAccountNumber(nabil!, "123")
    );

    const instruction: NpiPaymentInstruction = {
      instructionId: "TXN-TEST-001",
      debtorAgentBic: "STLRNPKA",
      creditorBankCode: "001",
      creditorSwift: "NABILNPKA",
      creditorName: "Suman Shrestha",
      creditorAccount: "00100175000010",
      amountNprPaisa: 13_485_000n,
      purposeCode: "FAMM",
      remittanceInfo: "Stellar Remit Test",
      createdAtIso: "2026-10-31T16:00:00Z",
    };

    const xml = toIso20022Pain001Xml(instruction);
    assert.ok(xml.includes("<InstdAmt Ccy=\"NPR\">134850.00</InstdAmt>"));
    assert.ok(xml.includes("<BIC>NABILNPKA</BIC>"));
  });

  it("verifies NRB compliance rules and statutory limits", () => {
    // Normal amount
    const normal = ComplianceEngine.verifyTransaction(
      13_485_000n,
      0n,
      0n,
      "Suman Shrestha"
    );
    assert.equal(normal.isCompliant, true);

    // Over 1 Million limit
    const excessive = ComplianceEngine.verifyTransaction(
      COMPLIANCE_LIMITS.MAX_SINGLE_TXN_PAISA + 100n,
      0n,
      0n,
      "Suman Shrestha"
    );
    assert.equal(excessive.isCompliant, false);
  });

  it("executes full end-to-end atomic remittance pipeline", () => {
    const rate = STANDARD_NRB_RATES.AED; // 2,000 AED sent from Dubai
    const req: RemittanceRequest = {
      stellarTxHash:
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      sourceAccount:
        "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
      destinationAccount:
        "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H",
      asset: CIRCLE_USDC,
      amountStroops: 2_000_000_000n,
      foreignCurrency: "AED",
      sourceAmountMinor: 200_000n, // 2,000.00 AED
      sourceDecimals: 2,
      beneficiaryName: "Gita Devi Thapa",
      beneficiaryBankQuery: "GLBINPKA", // Global IME Bank
      beneficiaryAccount: "00900175000010",
      purposeCode: "FAMM",
      senderPrior24hPaisa: 0n,
      senderPrior30dPaisa: 0n,
    };

    const receipt = RemittancePipeline.processRemittance(
      req,
      rate,
      "STLRNPKA",
      "CONNECTIPS_APP_001",
      "2026-10-31T16:00:00Z"
    );

    assert.equal(receipt.beneficiaryBank.swiftBic, "GLBINPKA");
    assert.ok(receipt.settlementNprPaisa > 0n);
    assert.ok(receipt.savingsSummary.familySavingsNprPaisa > 0n);
    assert.ok(receipt.cryptographicProofHash.length === 64);
  });
});
