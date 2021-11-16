import {readFileSync} from 'fs';
import {Secp256k1KeyIdentity} from '@dfinity/identity';
import sha256 from "sha256";

const getIdentityFromPem = (pem: string) => {
    const data = pem
        .replace('-----BEGIN PRIVATE KEY-----', '')
        .replace('-----END PRIVATE KEY-----', '')
        .trim();
    const arr = Array.from(data).map(letter => letter.charCodeAt(0));
    const rawBuffer = Uint8Array.from(arr);
    const privKey = Uint8Array.from(sha256(rawBuffer, {asBytes: true}));
    const identity = Secp256k1KeyIdentity.fromSecretKey(
        Uint8Array.from(privKey).buffer
    );
    return identity;
}

const getIdentityFromFile = (path: string) => {
    const buffer = readFileSync(path);
    const pem = buffer.toString();
    return getIdentityFromPem(pem);
}

export {getIdentityFromPem, getIdentityFromFile};
