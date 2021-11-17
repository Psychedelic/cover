const network = process.env.DFX_NETWORK || 'local';

/**
 * Get canister id from env or extract from
 * .dfx/local/canister_ids.json file
 * @return "rrkah-fqaaa-aaaaa-aaaaq-cai"
 */
const getCoverCanisterId = () => {
    const id = process.env.COVER_CANISTER_ID;

    // if (!id) {
    //     const net = network === 'local' ? 'local' : 'ic';
    //     // @todo: please replace this, do not use require outside of project directory
    //     // const canisters = require(`../../../../.dfx/${net}/canister_ids.json`);
    //     // id = canisters.cover[net] // returns canister_id
    // }
    if (process.env.DEBUG) {console.log('COVER_CANISTER_ID', id)};
    return id;
}

export {getCoverCanisterId};
