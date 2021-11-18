import 'source-map-support/register';

import middy from '@middy/core';
import createActor from '@libs/actor';
import type { SQSEvent } from 'aws-lambda';
import { ValidatedEventSQSEvent } from '@libs/sqs';
import sqsBatch from '@middy/sqs-partial-batch-failure';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';
import { CoverPaylod, CoverSchema } from '@libs/coverPayload';
import { Principal } from '@dfinity/principal';

/**
 * Consumes messages from queue.
 * If partial faliure occurs through promise.allSettled.
 * Deletes successful messages off queue.
 * Throws to keep failed messages on queue.
 *
 * @param event an event from source queue
 * @returns processes a list of events and returned all success and failed messages
 */
const publish = async (event: SQSEvent) => {
  const promises = event.Records.map(async ({ body }) => {
    const coverPayload: CoverPaylod = JSON.parse(body);
    const tempPayload = {
      canister_checksum: 'something',
      ...{
        ...coverPayload,
        canister_id: Principal.fromText(coverPayload.canister_id),
      },
    };

    console.log(tempPayload);

    try {
      const result = await createActor().add_verification(tempPayload);

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
