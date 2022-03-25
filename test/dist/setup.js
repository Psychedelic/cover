import { Actor, HttpAgent } from "@dfinity/agent";
import { Ed25519KeyIdentity } from "@dfinity/identity";
import fetch from "isomorphic-fetch";
import { idlFactory } from "./factory/idl.js";
import { readFileSync } from "fs";
export const aliceIdentity = Ed25519KeyIdentity.generate();
export const bobIdentity = Ed25519KeyIdentity.generate();
export const johnIdentity = Ed25519KeyIdentity.generate();
export const anotherAdminIdentity = Ed25519KeyIdentity.generate();
export const validatorIdentity = Ed25519KeyIdentity.generate();
export const builderIdentity = Ed25519KeyIdentity.generate();
const secretKey = readFileSync("./admin-test-secret", { encoding: "utf8" });
export const adminIdentity = Ed25519KeyIdentity.fromSecretKey(Buffer.from(secretKey, "hex"));
const canisterIds = JSON.parse(readFileSync("../canister_ids.json", { encoding: "utf8" }));
const createActor = async (identity) => {
    const agent = new HttpAgent({ host: "http://127.0.0.1:8000", fetch, identity });
    const actor = Actor.createActor(idlFactory, {
        canisterId: canisterIds.cover_test.local,
        agent
    });
    await agent.fetchRootKey().catch(err => {
        console.error("Unable to fetch root key. Check to ensure that your local replica is running");
        throw err;
    });
    return actor;
};
export const aliceActor = await createActor(aliceIdentity);
export const bobActor = await createActor(bobIdentity);
export const johnActor = await createActor(johnIdentity);
export const adminActor = await createActor(adminIdentity);
export const validatorActor = await createActor(validatorIdentity);
export const anotherAdminActor = await createActor(anotherAdminIdentity);
export const builderActor = await createActor(builderIdentity);
