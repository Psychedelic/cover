import 'source-map-support/register';

import type {ValidatedEventAPIGatewayProxyEvent} from '@libs/apiGateway';
import {formatJSONResponse} from '@libs/apiGateway';
import {middyfy} from '@libs/lambda';
import schema from './schema';
import createActor from '../../libs/actor';

const publish: ValidatedEventAPIGatewayProxyEvent<typeof schema> = async (event) => {
    const data = event.body.json;
    const ret = await createActor().add_verification(data)
        .then(json => ({
            statusCode: 200,
            body: JSON.stringify({message: 'success', json})
        }))
        .catch(err => ({
                // TODO: add server side error handling
                statusCode: 400, // Bad Request
                body: JSON.stringify({message: 'error', error: err.message})
            })
        );
    return ret;
}

export const main = middyfy(publish);
