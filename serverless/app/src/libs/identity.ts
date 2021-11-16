import { readFileSync } from "fs";
import { Secp256k1KeyIdentity } from "@dfinity/identity";

const PEM_BEGIN = "-----BEGIN PRIVATE KEY-----";
const PEM_END = "-----END PRIVATE KEY-----";

const PRIV_KEY_INIT =
  "308184020100301006072a8648ce3d020106052b8104000a046d306b0201010420";

const KEY_SEPARATOR = "a144034200";

const getIdentityFromPem = (pem: string) => {
  pem = pem.replace(PEM_BEGIN, "");
  pem = pem.replace(PEM_END, "");
  pem = pem.replace("\n", "");

  const pemBuffer = Buffer.from(pem, "base64");
  const pemHex = pemBuffer.toString("hex");

  const keys = pemHex.replace(PRIV_KEY_INIT, "");
  const [privateKey, publicKey] = keys.split(KEY_SEPARATOR);

  const identity = Secp256k1KeyIdentity.fromParsedJson([
    publicKey,
    privateKey.substr(-64),
  ]);

  return identity;
};

const getIdentityFromFile = (path: string) => {
  const buffer = readFileSync(path);
  const pem = buffer.toString();
  return getIdentityFromPem(pem);
};

export { getIdentityFromPem, getIdentityFromFile };
