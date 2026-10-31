/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
import {
  ComplianceEngine,
  type ComplianceReport,
  type RemittancePurposeCode,
} from "./compliance.ts";
import { sha256Hex } from "./crypto.ts";
import {
  type ForeignCurrency,
  FxEngine,
  type NrbExchangeRate,
  type RemittanceCostComparison,
} from "./fx.ts";
import {
  BankRegistry,
  type CommercialBank,
  computeConnectIpsSignature,
  type NpiPaymentInstruction,
} from "./nchl.ts";
import {
  isValidStellarAccountId,
  isValidStellarTxHash,
  type StellarAsset,
} from "./stellar.ts";

export interface RemittanceRequest {
  stellarTxHash: string;
  sourceAccount: string;
  destinationAccount: string;
  asset: StellarAsset;
  amountStroops: bigint;
  foreignCurrency: ForeignCurrency;
  sourceAmountMinor: bigint;
  sourceDecimals: number;
  beneficiaryName: string;
  beneficiaryBankQuery: string;
  beneficiaryAccount: string;
  purposeCode: RemittancePurposeCode;
  senderPrior24hPaisa: bigint;
  senderPrior30dPaisa: bigint;
}

export interface RemittanceReceipt {
  receiptId: string;
  stellarTxHash: string;
  beneficiaryBank: CommercialBank;
  beneficiaryAccount: string;
  settlementNprPaisa: bigint;
  savingsSummary: RemittanceCostComparison;
  npiInstruction: NpiPaymentInstruction;
  connectipsSignature: string;
  cryptographicProofHash: string;
  complianceReport: ComplianceReport;
}

export class RemittancePipeline {
  public static processRemittance(
    request: RemittanceRequest,
    rate: NrbExchangeRate,
    gatewayBic: string,
    appId: string,
    currentIsoTimestamp: string,
  ): RemittanceReceipt {
    if (!isValidStellarTxHash(request.stellarTxHash)) {
      throw new Error("Invalid Stellar transaction hash format");
    }
    if (!isValidStellarAccountId(request.sourceAccount)) {
      throw new Error("Invalid Stellar sender public key");
    }

    const bank = BankRegistry.findBank(request.beneficiaryBankQuery);
    if (!bank) {
      throw new Error(`Unknown Nepal commercial bank: ${request.beneficiaryBankQuery}`);
    }

    BankRegistry.validateAccountNumber(bank, request.beneficiaryAccount);

    // Exact FX & Family Savings Calculation (5 bps protocol fee vs 650 bps legacy)
    const costComparison = FxEngine.evaluateCostSavings(
      request.sourceAmountMinor,
      request.sourceDecimals,
      rate,
      5,
      650,
    );

    const settlementPaisa = costComparison.netRecipientNprPaisa;

    // Regulatory Compliance verification
    const complianceReport = ComplianceEngine.verifyTransaction(
      settlementPaisa,
      request.senderPrior24hPaisa,
      request.senderPrior30dPaisa,
      request.beneficiaryName,
    );

    if (!complianceReport.isCompliant) {
      throw new Error(
        `Regulatory compliance rejected: ${complianceReport.validationErrors.join("; ")}`,
      );
    }

    const instructionId = `NPI-${request.stellarTxHash.slice(0, 16)}`;
    const npiInstruction: NpiPaymentInstruction = {
      instructionId,
      debtorAgentBic: gatewayBic,
      creditorBankCode: bank.code,
      creditorSwift: bank.swiftBic,
      creditorName: request.beneficiaryName,
      creditorAccount: request.beneficiaryAccount,
      amountNprPaisa: settlementPaisa,
      purposeCode: request.purposeCode,
      remittanceInfo: `Stellar Remit Tx: ${request.stellarTxHash.slice(0, 12)}`,
      createdAtIso: currentIsoTimestamp,
    };

    const signature = computeConnectIpsSignature(
      appId,
      instructionId,
      settlementPaisa,
      request.stellarTxHash.slice(0, 12),
    );

    const auditPayload = `${request.stellarTxHash}:${bank.code}:${request.beneficiaryAccount}:${settlementPaisa}:${signature}`;
    const proofHash = sha256Hex(auditPayload);

    return {
      receiptId: `REMIT-${proofHash.slice(0, 16)}`,
      stellarTxHash: request.stellarTxHash,
      beneficiaryBank: bank,
      beneficiaryAccount: request.beneficiaryAccount,
      settlementNprPaisa: settlementPaisa,
      savingsSummary: costComparison,
      npiInstruction,
      connectipsSignature: signature,
      cryptographicProofHash: proofHash,
      complianceReport,
    };
  }
}
