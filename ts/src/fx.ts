/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

export type ForeignCurrency =
  | "USD"
  | "AED"
  | "QAR"
  | "SAR"
  | "MYR"
  | "EUR"
  | "GBP"
  | "AUD"
  | "JPY";

export interface NrbExchangeRate {
  currency: ForeignCurrency;
  unit: number;
  buyingRatePaisa: bigint;
  sellingRatePaisa: bigint;
  midRatePaisa: bigint;
}

export function createNrbExchangeRate(
  currency: ForeignCurrency,
  unit: number,
  buyingRatePaisa: bigint,
  sellingRatePaisa: bigint,
): NrbExchangeRate {
  return {
    currency,
    unit,
    buyingRatePaisa,
    sellingRatePaisa,
    midRatePaisa: (buyingRatePaisa + sellingRatePaisa) / 2n,
  };
}

export const STANDARD_NRB_RATES: Record<ForeignCurrency, NrbExchangeRate> = {
  USD: createNrbExchangeRate("USD", 1, 13485n, 13545n), // 134.85 NPR
  AED: createNrbExchangeRate("AED", 1, 3672n, 3688n),   // 36.72 NPR
  QAR: createNrbExchangeRate("QAR", 1, 3704n, 3721n),   // 37.04 NPR
  SAR: createNrbExchangeRate("SAR", 1, 3594n, 3610n),   // 35.94 NPR
  MYR: createNrbExchangeRate("MYR", 1, 3110n, 3125n),   // 31.10 NPR
  EUR: createNrbExchangeRate("EUR", 1, 14620n, 14685n), // 146.20 NPR
  GBP: createNrbExchangeRate("GBP", 1, 17540n, 17618n), // 175.40 NPR
  AUD: createNrbExchangeRate("AUD", 1, 8910n, 8950n),   // 89.10 NPR
  JPY: createNrbExchangeRate("JPY", 10, 895n, 901n),    // 89.50 NPR per 10
};

export interface RemittanceCostComparison {
  foreignCurrency: ForeignCurrency;
  foreignAmountMinor: bigint;
  grossNprPaisa: bigint;
  protocolFeeBps: number;
  protocolFeeNprPaisa: bigint;
  netRecipientNprPaisa: bigint;
  legacyFeePercentBps: number;
  legacyFeeNprPaisa: bigint;
  legacyRecipientNprPaisa: bigint;
  familySavingsNprPaisa: bigint;
}

export class FxEngine {
  /**
   * Converts foreign minor units (e.g. cents) to NPR paisa using exact integer arithmetic
   */
  public static convertToNprPaisa(
    foreignAmountMinor: bigint,
    foreignDecimals: number,
    rate: NrbExchangeRate,
  ): bigint {
    const scale = 10n ** BigInt(foreignDecimals);
    const numerator = foreignAmountMinor * rate.buyingRatePaisa;
    const denominator = scale * BigInt(rate.unit);
    return numerator / denominator;
  }

  /**
   * Evaluates cost breakdown and direct family savings in NPR
   */
  public static evaluateCostSavings(
    foreignAmountMinor: bigint,
    foreignDecimals: number,
    rate: NrbExchangeRate,
    protocolFeeBps: number = 5,   // 0.05%
    legacyFeeBps: number = 650,   // 6.5%
  ): RemittanceCostComparison {
    const grossNprPaisa = this.convertToNprPaisa(foreignAmountMinor, foreignDecimals, rate);

    const protocolFeeNprPaisa = (grossNprPaisa * BigInt(protocolFeeBps)) / 10000n;
    const netRecipientNprPaisa =
      grossNprPaisa > protocolFeeNprPaisa ? grossNprPaisa - protocolFeeNprPaisa : 0n;

    const legacyFeeNprPaisa = (grossNprPaisa * BigInt(legacyFeeBps)) / 10000n;
    const legacyRecipientNprPaisa =
      grossNprPaisa > legacyFeeNprPaisa ? grossNprPaisa - legacyFeeNprPaisa : 0n;

    const familySavingsNprPaisa =
      netRecipientNprPaisa > legacyRecipientNprPaisa
        ? netRecipientNprPaisa - legacyRecipientNprPaisa
        : 0n;

    return {
      foreignCurrency: rate.currency,
      foreignAmountMinor,
      grossNprPaisa,
      protocolFeeBps,
      protocolFeeNprPaisa,
      netRecipientNprPaisa,
      legacyFeePercentBps: legacyFeeBps,
      legacyFeeNprPaisa,
      legacyRecipientNprPaisa,
      familySavingsNprPaisa,
    };
  }

  public static formatNpr(paisa: bigint): string {
    const rupees = paisa / 100n;
    const p = paisa % 100n;
    const pStr = p < 10n ? `0${p}` : `${p}`;
    return `${rupees}.${pStr} NPR`;
  }
}
