import { Actor, HttpAgent } from '@dfinity/agent';
import { CoverIDL, CoverIDLFactory } from '../idls';
import fetch from 'node-fetch';
import { getCoverCanisterId } from './utils';
import { getIdentityFromFile, getIdentityFromPem } from './identity';
let identity: any;
if (process.env.IDENTITY_PEM) {
  if (process.env.DEBUG)
    console.log('Using identity.pem string', process.env.IDENTITY_PEM);
  identity = getIdentityFromPem(process.env.IDENTITY_PEM);
} else if (process.env.IDENTITY_PEM_PATH) {
  // use path
  if (process.env.DEBUG)
    console.log(`Using identity from file ${process.env.IDENTITY_PEM_PATH}`);
  identity = getIdentityFromFile(process.env.IDENTITY_PEM_PATH);
} else {
  throw new Error('Either IDENTITY_PEM or IDENTITY_PEM_PATH is required');
}

const createActor = () => {
  const canisterId = getCoverCanisterId();

  // @ts-ignore
  const agent = new HttpAgent({
    host: 'http://127.0.0.1:8000',
    fetch,
    identity,
  });

  // Fetch root key for certificate validation during development
  if (process.env.NODE_ENV !== 'production') {
    agent.fetchRootKey().catch((err) => {
      console.error(
        'Unable to fetch root key. Check to ensure that your local replica is running'
      );
      throw err;
    });
  }

  return Actor.createActor<CoverIDL>(CoverIDLFactory, {
    canisterId,
    agent,
  });
};

export default createActor;
