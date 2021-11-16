import 'source-map-support/register';

import type {ValidatedEventAPIGatewayProxyEvent} from '@libs/apiGateway';
import {middyfy} from '@libs/lambda';
import schema from './schema';
import createActor from '../../libs/actor';

const publish: ValidatedEventAPIGatewayProxyEvent<typeof schema> = async (event) => {
    return createActor().add_verification(event.body.json)
        .then(json => ({
            statusCode: 200,
            body: JSON.stringify({message: 'success', json})
        }))
        .catch(err => ({
                // TODO: add server side error handling with status
                statusCode: 400, // Bad Request
                body: JSON.stringify({message: 'error', error: err.message})
            })
        );
}

export const main = middyfy(publish);
