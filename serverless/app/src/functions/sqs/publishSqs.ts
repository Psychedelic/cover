import 'source-map-support/register';

import {middyfy} from '@libs/lambda';
import {CoverPayload} from '@functions/sqs/coverPayload';
import {APIGatewayProxyHandler} from 'aws-lambda';
import {formatJSONResponse} from '@libs/apiGateway';
import {
    SQSClient,
    SendMessageCommand,
} from '@aws-sdk/client-sqs';


let QueueUrl = process.env.QUEUE_URL;
if (process.env.STAGE == 'local') {
    QueueUrl = `http://localhost:9324/queue/cover-queue-cover-${process.env.STAGE}`
}

const sqsClient = new SQSClient({region: process.env.AWS_REGION});

export const publishSqs: APIGatewayProxyHandler = async (
    event
): Promise<any> => {
    if (!event.body) {
        return formatJSONResponse({
            statusCode: 500,
            body: `Error publishSqs: no data!`,
        });
    }

    const coverPayload = event.body as unknown as CoverPayload;

    const response = {
        statusCode: 200,
        body: ''
    };
    try {
        console.log('Sending', {
            // env: process.env,
            QueueUrl, coverPayload
        });
        const command = new SendMessageCommand({
            QueueUrl,
            MessageBody: JSON.stringify(coverPayload),
            // MessageGroupId: mockEvent.git_checksum,
            // MessageDeduplicationId: uuid(),
        });
        await sqsClient.send(command);
    } catch (e) {
        console.error('Exception on queue', e);
        response.body = `Error on send queue: ${e}`;
        response.statusCode = 500;
    }

    return formatJSONResponse(response);
};

export const main = middyfy(publishSqs);
