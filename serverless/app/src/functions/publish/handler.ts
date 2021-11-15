import 'source-map-support/register';

import type {ValidatedEventAPIGatewayProxyEvent} from '@libs/apiGateway';
import {formatJSONResponse} from '@libs/apiGateway';
import {middyfy} from '@libs/lambda';
import schema from './schema';
import createActor from '../../libs/actor';

const publish: ValidatedEventAPIGatewayProxyEvent<typeof schema> = async (event) => {
    const data = event.body.json;
    const ret = await createActor().add_verification(data)
        .then(json =>
            formatJSONResponse({message: 'success', json})
        )
        .catch(err => formatJSONResponse({
                message: 'error',
                error: err.message,
            })
        );
    return ret;
}

export const main = middyfy(publish);
