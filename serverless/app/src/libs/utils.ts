/**
 * Get canister id from env or extract from
 * .dfx/local/canister_ids.json file
 * @return "rrkah-fqaaa-aaaaa-aaaaq-cai"
 */
const getCoverCanisterId = () => {
    const id = process.env.COVER_CANISTER_ID;
    if (!id) {
        throw new Error("COVER_CANISTER_ID missing!");
    }
    if (process.env.DEBUG) {
        console.log('COVER_CANISTER_ID', id)
    }
    return id;
}

export {getCoverCanisterId};
