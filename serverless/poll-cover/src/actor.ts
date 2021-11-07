import {Actor, HttpAgent} from "@dfinity/agent";
import {CoverIDL, CoverIDLFactory} from "./idls";
import fetch from 'node-fetch';
import {getCanisterId} from "./utils";
import {getIdentityFromFile} from "./indentity";

const identityPem = process.env.IDENTITY_PEM;
const identity = getIdentityFromFile(identityPem);

const createActor = () => {
    const canisterId = getCanisterId();

    // @ts-ignore
    const agent = new HttpAgent({host: 'http://127.0.0.1:8000', fetch, identity});

    // Fetch root key for certificate validation during development
    if (process.env.NODE_ENV !== "production") {
        agent.fetchRootKey().catch(err => {
            console.error("Unable to fetch root key. Check to ensure that your local replica is running");
            throw err;
        });
    }

    return Actor.createActor<CoverIDL>(CoverIDLFactory, {
        canisterId,
        agent,
    });
};

export default createActor;
