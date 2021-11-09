import 'source-map-support/register';

import { formatJSONResponse } from '@libs/apiGateway';
import { middyfy } from '@libs/lambda';

const consume = async () => {
  // @todo: publish event to SQS
  // that SQS will be handled by another lambda
  // so we guarantee 100% delivery (or DLQ)

  return formatJSONResponse({
    message: `Consume called`
  });
}

export const main = middyfy(consume);
