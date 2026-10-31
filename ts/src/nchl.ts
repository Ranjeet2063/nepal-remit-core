/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */
import { sha256Hex } from "./crypto.ts";

export interface CommercialBank {
  code: string;
  swiftBic: string;
  nameEn: string;
  nameNe: string;
  minAccLen: number;
  maxAccLen: number;
}

export const COMMERCIAL_BANKS: CommercialBank[] = [
  {
    code: "001",
    swiftBic: "NABILNPKA",
    nameEn: "Nabil Bank Limited",
    nameNe: "नबिल बैंक लिमिटेड",
    minAccLen: 13,
    maxAccLen: 16,
  },
  {
    code: "002",
    swiftBic: "NIBLNPKT",
    nameEn: "Nepal Investment Mega Bank Limited",
    nameNe: "नेपाल इन्भेष्टमेण्ट मेगा बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "003",
    swiftBic: "SCBLNPKA",
    nameEn: "Standard Chartered Bank Nepal Limited",
    nameNe: "स्ट्यान्डर्ड चार्टर्ड बैंक नेपाल लिमिटेड",
    minAccLen: 13,
    maxAccLen: 15,
  },
  {
    code: "004",
    swiftBic: "HIMANPKA",
    nameEn: "Himalayan Bank Limited",
    nameNe: "हिमालयन बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "005",
    swiftBic: "NSBINPKT",
    nameEn: "Nepal SBI Bank Limited",
    nameNe: "नेपाल एसबिआई बैंक लिमिटेड",
    minAccLen: 16,
    maxAccLen: 18,
  },
  {
    code: "006",
    swiftBic: "EVBLNPKA",
    nameEn: "Everest Bank Limited",
    nameNe: "एभरेष्ट बैंक लिमिटेड",
    minAccLen: 12,
    maxAccLen: 16,
  },
  {
    code: "007",
    swiftBic: "PRBUNPKA",
    nameEn: "Prabhu Bank Limited",
    nameNe: "प्रभु बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "008",
    swiftBic: "LXMINPKA",
    nameEn: "Laxmi Sunrise Bank Limited",
    nameNe: "लक्ष्मी सनराइज बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "009",
    swiftBic: "GLBINPKA",
    nameEn: "Global IME Bank Limited",
    nameNe: "ग्लोबल आइएमई बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 17,
  },
  {
    code: "010",
    swiftBic: "CZBINPKA",
    nameEn: "Citizens Bank International Limited",
    nameNe: "सिटिजन्स बैंक इन्टरनेसनल लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "011",
    swiftBic: "PCBLNPKA",
    nameEn: "Prime Commercial Bank Limited",
    nameNe: "प्राइम कमर्सियल बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "012",
    swiftBic: "SNBLNPKA",
    nameEn: "Sunrise Bank Limited",
    nameNe: "सनराइज बैंक लिमिटेड",
    minAccLen: 13,
    maxAccLen: 15,
  },
  {
    code: "013",
    swiftBic: "KMBLNPKA",
    nameEn: "Kumari Bank Limited",
    nameNe: "कुमारी बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "014",
    swiftBic: "SIDDNPKA",
    nameEn: "Siddhartha Bank Limited",
    nameNe: "सिद्धार्थ बैंक लिमिटेड",
    minAccLen: 13,
    maxAccLen: 16,
  },
  {
    code: "015",
    swiftBic: "ADBLNPKA",
    nameEn: "Agriculture Development Bank Limited",
    nameNe: "कृषि विकास बैंक लिमिटेड",
    minAccLen: 16,
    maxAccLen: 18,
  },
  {
    code: "016",
    swiftBic: "NICAINPKA",
    nameEn: "NIC Asia Bank Limited",
    nameNe: "एनआईसी एशिया बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "017",
    swiftBic: "MBLINPKT",
    nameEn: "Machhapuchchhre Bank Limited",
    nameNe: "माछापुच्छ्रे बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "018",
    swiftBic: "SNMANPKA",
    nameEn: "Sanima Bank Limited",
    nameNe: "सानिमा बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
  {
    code: "019",
    swiftBic: "RBBANPKA",
    nameEn: "Rastriya Banijya Bank Limited",
    nameNe: "राष्ट्रिय वाणिज्य बैंक लिमिटेड",
    minAccLen: 12,
    maxAccLen: 16,
  },
  {
    code: "020",
    swiftBic: "NEBLNPKA",
    nameEn: "Nepal Bank Limited",
    nameNe: "नेपाल बैंक लिमिटेड",
    minAccLen: 14,
    maxAccLen: 16,
  },
];

export class BankRegistry {
  public static findBank(query: string): CommercialBank | undefined {
    const q = query.trim().toUpperCase();
    return COMMERCIAL_BANKS.find(
      (b) =>
        b.code === query.trim() ||
        b.swiftBic === q ||
        b.nameEn.toUpperCase().includes(q),
    );
  }

  public static validateAccountNumber(
    bank: CommercialBank,
    accountNumber: string,
  ): void {
    const clean = accountNumber.trim();
    if (!/^\d+$/.test(clean)) {
      throw new Error("Bank account number must contain digits only");
    }
    if (clean.length < bank.minAccLen || clean.length > bank.maxAccLen) {
      throw new Error(
        `Bank account number length (${clean.length}) out of range [${bank.minAccLen}-${bank.maxAccLen}] for ${bank.nameEn}`,
      );
    }
  }
}

export interface NpiPaymentInstruction {
  instructionId: string;
  debtorAgentBic: string;
  creditorBankCode: string;
  creditorSwift: string;
  creditorName: string;
  creditorAccount: string;
  amountNprPaisa: bigint;
  purposeCode: string;
  remittanceInfo: string;
  createdAtIso: string;
}

export function computeConnectIpsSignature(
  appId: string,
  txnId: string,
  amountPaisa: bigint,
  refId: string,
): string {
  const rupees = amountPaisa / 100n;
  const paisa = amountPaisa % 100n;
  const pStr = paisa < 10n ? `0${paisa}` : `${paisa}`;
  const rawPayload = `appId=${appId},txnId=${txnId},txnAmt=${rupees}.${pStr},refId=${refId}`;
  return sha256Hex(rawPayload);
}

export function toIso20022Pain001Xml(instr: NpiPaymentInstruction): string {
  const rupees = instr.amountNprPaisa / 100n;
  const p = instr.amountNprPaisa % 100n;
  const amountStr = `${rupees}.${p < 10n ? "0" + p : p}`;
  const reqDate = instr.createdAtIso.slice(0, 10);

  return `<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pain.001.001.03">
  <CstmrCdtTrfInitn>
    <GrpHdr>
      <MsgId>${instr.instructionId}</MsgId>
      <CreDtTm>${instr.createdAtIso}</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <CtrlSum>${amountStr}</CtrlSum>
      <InitgPty>
        <Nm>Stellar-Nepal-Remittance-Gateway</Nm>
      </InitgPty>
    </GrpHdr>
    <PmtInf>
      <PmtInfId>PMT-${instr.instructionId}</PmtInfId>
      <PmtMtd>TRF</PmtMtd>
      <ReqdExctnDt>${reqDate}</ReqdExctnDt>
      <Dbtr>
        <Nm>NonCustodial Remittance Bridge</Nm>
      </Dbtr>
      <DbtrAgt>
        <FinInstnId>
          <BIC>${instr.debtorAgentBic}</BIC>
        </FinInstnId>
      </DbtrAgt>
      <CdtTrfTxInf>
        <PmtId>
          <EndToEndId>${instr.instructionId}</EndToEndId>
        </PmtId>
        <Amt>
          <InstdAmt Ccy="NPR">${amountStr}</InstdAmt>
        </Amt>
        <CdtrAgt>
          <FinInstnId>
            <BIC>${instr.creditorSwift}</BIC>
            <ClrSysMmbId>
              <MmbId>${instr.creditorBankCode}</MmbId>
            </ClrSysMmbId>
          </FinInstnId>
        </CdtrAgt>
        <Cdtr>
          <Nm>${instr.creditorName}</Nm>
        </Cdtr>
        <CdtrAcct>
          <Id>
            <Othr>
              <Id>${instr.creditorAccount}</Id>
            </Othr>
          </Id>
        </CdtrAcct>
        <RmtInf>
          <Ustrd>${instr.remittanceInfo}</Ustrd>
        </RmtInf>
      </CdtTrfTxInf>
    </PmtInf>
  </CstmrCdtTrfInitn>
</Document>`;
}
