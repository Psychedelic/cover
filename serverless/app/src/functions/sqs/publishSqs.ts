import 'source-map-support/register';

import { v4 as uuid } from 'uuid';
import { middyfy } from '@libs/lambda';
import { CoverPayload } from '@libs/coverPayload';
import { APIGatewayProxyHandler } from 'aws-lambda';
import { formatJSONResponse } from '@libs/apiGateway';
import {
  SQSClient,
  SendMessageBatchCommand,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';

const LOCAL = true;

// const QueueUrl = LOCAL
//   ? `http://localhost:9324/queue/cover-queue-cover-${process.env.STAGE}`
//   : (process.env.QUEUE_URL as string);

// const QueueUrl = process.env.QUEUE_URL;
const QueueUrl = 'https://sqs.us-west-2.amazonaws.com/768127979193/cover-queue-cover-dev';

const MAX_MESSAGES_PER_BATCH = 10;
const sqsClient = new SQSClient({ region: process.env.AWS_REGION });

export const publishSqs: APIGatewayProxyHandler = async (
  event
): Promise<any> => {
  if (!QueueUrl) {
    throw new Error('No queue URL present in environment')
  }

  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: `Error publishSqs: no data!`,
    });
  }

  const coverPayload: CoverPayload = event.body;
  console.log(coverPayload);

  const promises = []
  const command = new SendMessageCommand({
    QueueUrl,
    MessageBody: JSON.stringify(coverPayload),
    // MessageGroupId: mockEvent.git_checksum,
    // MessageDeduplicationId: uuid(),
  });

  promises.push(sqsClient.send(command));

  const failed = (await Promise.allSettled(promises))
    .flat()
    .filter(({ status }) => status === 'rejected')

  if (failed.length) {
    throw new Error(`${failed.length} commands could not be sent ${JSON.stringify(failed)}`)
  }

  return formatJSONResponse({ status: 'OK' })
};

export const main = middyfy(publishSqs);
