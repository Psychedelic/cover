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

const QueueUrl =
  'https://sqs.us-west-2.amazonaws.com/768127979193/cover-queue-cover-dev';

const MAX_MESSAGES_PER_BATCH = 10;
const sqsClient = new SQSClient({ region: process.env.AWS_REGION });

const mockEvent = {
  canister_id: 'REMOVE',
  created_at: '2021/11/15_11:28:01:103133356',
  git_ref: 'refs/heads/feat/github-plugin',
  git_checksum: '6d55a6d3288c708e0a68d8ac8c6277b2bbff3ff1',
  source_snapshot_url: 'N/A',
  wasm_path: 'services/cover/Cargo.toml',
  wasm_checksum:
    '0x4d80d6cd59573d16b368929d0754efb5b98eb7ffaaab6d4464218e25f8aaedf3',
  build_log_url: 'TODO',
};

export const publishSqs: APIGatewayProxyHandler = async (): Promise<any> => {
  const responseBody = {
    responseCode: 200,
    message: '',
    messageId: '',
  };

  try {
    const command = new SendMessageCommand({
      QueueUrl,
      MessageBody: JSON.stringify(mockEvent),
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
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(responseBody),
  };

  return formatJSONResponse(response);
};

export const main = middyfy(publishSqs);
