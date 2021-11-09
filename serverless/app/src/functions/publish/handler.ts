import 'source-map-support/register';

import type { ValidatedEventAPIGatewayProxyEvent } from '@libs/apiGateway';
import { formatJSONResponse } from '@libs/apiGateway';
import { middyfy } from '@libs/lambda';

import schema from './schema';

const publish: ValidatedEventAPIGatewayProxyEvent<typeof schema> = async (event) => {
  console.log(event);

  // @todo: publish event to SQS
  // that SQS will be handled by another lambda
  // so we guarantee 100% delivery (or DLQ)

  return formatJSONResponse({
    message: `Hello ${event.body.name}, welcome to the exciting Serverless world!`,
    event,
  });
}

export const main = middyfy(publish);
