import {readFileSync} from 'fs';
import {Secp256k1KeyIdentity} from '@dfinity/identity';

const PRIV_KEY_INIT =
    '308184020100301006072a8648ce3d020106052b8104000a046d306b0201010420';
const KEY_SEPARATOR = 'a144034200';

const getIdentityFromPem = (pem: string): Secp256k1KeyIdentity => {
    pem = pem
        .replace('-----BEGIN PRIVATE KEY-----', '')
        .replace('-----END PRIVATE KEY-----', '')
        .trim();

    const pemBuffer = Buffer.from(pem, 'base64');
    const pemHex = pemBuffer.toString('hex');

    const keys = pemHex.replace(PRIV_KEY_INIT, '');
    const [privateKey, publicKey] = keys.split(KEY_SEPARATOR);

    if (!publicKey) throw new Error(`Invalid identity publicKey: ${pem}`);
    if (!privateKey) throw new Error(`Invalid identity privateKey: ${pem}`);

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

const getIdentityFromEnv = () => {
    let identity: any;
    let pem = process.env.LOCAL_IDENTITY_PEM;
    if (process.env.DEBUG && pem) {
        console.log('Using LOCAL_IDENTITY_PEM');
    }
    if (!pem) {
        pem = process.env.IDENTITY_PEM;
        if (process.env.DEBUG && pem) console.log('Using IDENTITY_PEM');
    }

    if (pem) {
        identity = getIdentityFromPem(pem);
    } else if (process.env.IDENTITY_PEM_PATH) {
        // use path
        if (process.env.DEBUG)
            console.log(`Using IDENTITY_PEM_PATH ${process.env.IDENTITY_PEM_PATH}`);
        identity = getIdentityFromFile(process.env.IDENTITY_PEM_PATH);
    } else {
        throw new Error('Either IDENTITY_PEM or IDENTITY_PEM_PATH is required');
    }
    return identity;
}
export {getIdentityFromPem, getIdentityFromFile, getIdentityFromEnv};
