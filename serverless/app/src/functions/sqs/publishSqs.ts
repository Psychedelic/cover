import 'source-map-support/register';

import { v4 as uuid } from 'uuid';
import { middyfy } from '@libs/lambda';
import { CoverPayload } from '@libs/coverPayload';
import { APIGatewayProxyHandler } from 'aws-lambda';
import { formatJSONResponse } from '@libs/apiGateway';
import {
  SQSClient,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';

const QueueUrl = process.env.QUEUE_URL;
// const QueueUrl = `http://localhost:9324/queue/cover-queue-cover-${process.env.STAGE}`
// const QueueUrl = 'https://sqs.us-west-2.amazonaws.com/768127979193/cover-queue-cover-dev';

const sqsClient = new SQSClient({ region: process.env.AWS_REGION });

export const publishSqs: APIGatewayProxyHandler = async (
  event
): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: `Error publishSqs: no data!`,
    });
  }

  const coverPayload: CoverPayload = event.body;
  console.log('Received', { QueueUrl, coverPayload });

  const body = {
        responseCode: 200,
        message: '',
        messageId: '',
      };
  try {
    const command = new SendMessageCommand({
      QueueUrl,
      MessageBody: JSON.stringify(coverPayload),
      // MessageGroupId: mockEvent.git_checksum,
      // MessageDeduplicationId: uuid(),
    });

    return sqsClient.send(command);
  } catch (e) {
    responseBody.responseCode = 500;
    console.error('Exception on queue', e);
  }

  const response = {
    statusCode: 200,
    body;
  };

  return formatJSONResponse(response);
};

export const main = middyfy(publishSqs);
