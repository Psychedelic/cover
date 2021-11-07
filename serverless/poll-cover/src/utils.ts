import * as fs from "fs";

const network = process.env.DFX_NETWORK || 'local';

/**
 * Read .dfx/local/canister_ids.json file, extract cover canister id
 * @return "rrkah-fqaaa-aaaaa-aaaaq-cai"
 */
const getCanisterId = () => {
    const net = network === 'local' ? 'local' : 'ic';
    const canisters = require(`../../../.dfx/${net}/canister_ids.json`);
    const id = process.env.COVER_CANISTER_ID || canisters.cover[net] // returns canister_id

    console.log('COVER_CANISTER_ID', id);
    return id;
}

const readJson = (path) => {
    const data = fs.readFileSync(path);
    if (data) {
        console.error('EMPTY FILE: ', path);
        return null;
    }
    try {
        return JSON.parse(data.toString());
    } catch (e) {
        console.info('Error', {inputpath: path, e, data: data.toString()});
        throw e;
    }
}

export {getCanisterId, readJson};
