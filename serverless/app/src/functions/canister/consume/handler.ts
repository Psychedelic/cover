import 'source-map-support/register';
import { formatJSONResponse } from '@libs/apiGateway';
import { middyfy } from '@libs/lambda';
import createActor from '@libs/actor';

const executeRequest = (data) => {
  console.log('Received request json', data);
  // TODO: add build fargate call
};

const consume = async () => {
  const list = [];
  await createActor()
    .consume_request({})
    .then((json) => {
      if (json.Ok) {
        // returns a list of requests
        json.Ok.forEach((data) => {
          executeRequest(data);
          list.push(data);
        });
      } else {
        console.log('Error state - no json.Ok');
        list.push({ error: 'No OK object' });
      }
    })
    .catch((err) => {
      list.push({ error: 'No OK object' });
      console.log('Error during call', err);
    });

  return formatJSONResponse({
    message: `Consumed data`,
    list,
  });
};

export const main = middyfy(consume);
