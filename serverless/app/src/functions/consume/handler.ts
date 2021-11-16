import 'source-map-support/register';
import {formatJSONResponse} from '@libs/apiGateway';
import {middyfy} from '@libs/lambda';
import createActor from '../../libs/actor';

const executeRequest = (data) => {
    console.log("Received request json", data);
    // TODO: add build fargate call
}

// TODO: it's just an example, change to get_requests when ready
const consume = async () => {
    const list = [];
    await createActor().get_all_verifications()
        .then(vec => {
                vec.forEach(data => {
                    executeRequest(data);
                    list.push(data);
                });
            }
        )
        .catch(err => {
                list.push({"error": "No OK object"});
                console.log('Error during call', err);
            }
        );

    return formatJSONResponse({
        message: `Consumed data`,
        list
    });
}

export const main = middyfy(consume);
