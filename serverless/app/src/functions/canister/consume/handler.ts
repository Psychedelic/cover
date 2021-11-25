import 'source-map-support/register';
import {formatJSONResponse} from '@libs/apiGateway';
import {middyfy} from '@libs/lambda';
import {validRepoAccessToken} from "@libs/github";


// Just a tester lambda
const consume = async (event: any) => {
    console.log("Received event", {event});

    const ret = await validRepoAccessToken(event.body);
    console.log('Returned', ret );

    return formatJSONResponse({
        message: `Consumed data`,
        ret
    });
};

export const main = middyfy(consume);
