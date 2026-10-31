/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
import { createHash } from "node:crypto";

export function sha256Hex(data: string | Uint8Array): string {
  const hash = createHash("sha256");
  hash.update(data);
  return hash.digest("hex");
}
