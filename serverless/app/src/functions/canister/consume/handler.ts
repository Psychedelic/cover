import 'source-map-support/register';
import {formatJSONResponse} from '@libs/apiGateway';
import {middyfy} from '@libs/lambda';
import createActor from '@libs/actor';
import {Verification} from "../../../idls/cover.did";

// Just a placeholder for a future use
const consume = async () => {
    let list = [] as Verification[];
    await createActor()
        .get_all_verifications()
        .then((vers) => {
            list = vers;
            console.log('Verifications', vers);
        })
        .catch((err) => {
            console.log('Error during call', err);
        });

    return formatJSONResponse({
        message: `Consumed data`,
        list,
    });
};

export const main = middyfy(consume);
