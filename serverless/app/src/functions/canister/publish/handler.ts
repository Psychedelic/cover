import 'source-map-support/register';

import middy from '@middy/core';
import createActor from '@libs/actor';
import { ValidatedEventSQSEvent } from '@libs/sqs';
import sqsBatch from '@middy/sqs-partial-batch-failure';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';

const schema = require('./schema.json');

/**
 * Consumes messages from queue
 * If partial faliure occurs through promise.allSettled
 * Deletes successful messages off queue
 * Throws to keep failed messages on queue
 *
 * @param event an event from source queue
 * @returns processes a list of events and returned all success and failed messages
 */
const publish: ValidatedEventSQSEvent<typeof schema> = async (event) => {
  const promises = event.Records.map(async (record) => {
    const {
      body: { json },
    } = record;

    try {
      const result = await createActor().add_verification(json);

      return Promise.resolve({
        statusCode: 200,
        body: JSON.stringify({ message: 'success', result }),
      });
    } catch (error) {
      console.error(`Error publish:  ${(error as Error).message}`);
      return Promise.reject(error);
    }
  });

  return Promise.allSettled(promises);
};

export const main = middy(publish).use(sqsJsonBodyParser()).use(sqsBatch());
