/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
import { sha256Hex } from "./crypto.ts";

export type StellarAssetType = "XLM" | "USDC" | "EURC" | "CUSTOM";

export interface StellarAsset {
  type: StellarAssetType;
  code: string;
  issuer?: string;
}

export const CIRCLE_USDC: StellarAsset = {
  type: "USDC",
  code: "USDC",
  issuer: "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
};

export const CIRCLE_EURC: StellarAsset = {
  type: "EURC",
  code: "EURC",
  issuer: "GD6WVYRVID442Y4JVWFWKWCZTRDGIIZM3FIQIJWJFD7WW7RUM2L7CJ2W",
};

export const NATIVE_XLM: StellarAsset = {
  type: "XLM",
  code: "XLM",
};

export function isValidStellarAccountId(accountId: string): boolean {
  if (accountId.length !== 56 || !accountId.startsWith("G")) {
    return false;
  }
  return /^[A-Z2-7]{56}$/.test(accountId);
}

export function isValidStellarTxHash(hash: string): boolean {
  return /^[0-9a-fA-F]{64}$/.test(hash);
}

export type EscrowStatus =
  | { type: "Initiated" }
  | { type: "Funded" }
  | { type: "Claimed"; preimage: string; settledAtUnix: number }
  | { type: "Refunded"; refundedAtUnix: number };

export class RemittanceEscrow {
  public escrowId: string;
  public sender: string;
  public recipientNepalBank: string;
  public recipientAccount: string;
  public asset: StellarAsset;
  public amountStroops: bigint;
  public hashLock: string;
  public timeoutUnix: number;
  public status: EscrowStatus = { type: "Initiated" };

  constructor(
    escrowId: string,
    sender: string,
    recipientNepalBank: string,
    recipientAccount: string,
    asset: StellarAsset,
    amountStroops: bigint,
    hashLock: string,
    timeoutUnix: number,
  ) {
    if (!isValidStellarAccountId(sender)) {
      throw new Error("Invalid sender Stellar account ID");
    }
    if (amountStroops <= 0n) {
      throw new Error("Amount must be greater than zero");
    }
    this.escrowId = escrowId;
    this.sender = sender;
    this.recipientNepalBank = recipientNepalBank;
    this.recipientAccount = recipientAccount;
    this.asset = asset;
    this.amountStroops = amountStroops;
    this.hashLock = hashLock;
    this.timeoutUnix = timeoutUnix;
  }

  public markFunded(): void {
    if (this.status.type !== "Initiated") {
      throw new Error("Can only fund an initiated escrow");
    }
    this.status = { type: "Funded" };
  }

  public claim(preimage: string, currentUnix: number): void {
    if (this.status.type !== "Funded") {
      throw new Error("Escrow must be in Funded state to claim");
    }
    if (currentUnix > this.timeoutUnix) {
      throw new Error("Escrow timeout has expired; funds must be refunded to sender");
    }
    const computedHash = sha256Hex(preimage);
    if (computedHash.toLowerCase() !== this.hashLock.toLowerCase()) {
      throw new Error("Invalid preimage: SHA-256 hash does not match hashlock");
    }
    this.status = { type: "Claimed", preimage, settledAtUnix: currentUnix };
  }

  public refund(currentUnix: number): void {
    if (this.status.type !== "Funded") {
      throw new Error("Escrow must be in Funded state to refund");
    }
    if (currentUnix <= this.timeoutUnix) {
      throw new Error("Cannot refund before timeout expiration");
    }
    this.status = { type: "Refunded", refundedAtUnix: currentUnix };
  }
}
