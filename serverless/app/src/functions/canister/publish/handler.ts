import 'source-map-support/register';

import middy from '@middy/core';
import createActor from '@libs/actor';
import type {SQSEvent} from 'aws-lambda';
import sqsBatch from '@middy/sqs-partial-batch-failure';
import sqsJsonBodyParser from '@middy/sqs-json-body-parser';
import {CoverPayloadI} from '@functions/sqs/coverPayload';
import {Principal} from '@dfinity/principal';
import {formatJSONResponse} from "@libs/apiGateway";
import {AddVerification} from "../../../idls/cover.did";

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
    const promises = event.Records.map(async ({body}) => {
        const data = body as unknown as CoverPayloadI;
        const tempPayload = {
            ...data,
            canister_id: Principal.fromText(data.canister_id),
        };

        try {
            const payload = tempPayload as AddVerification;
            console.log('AddVerification', {payload});
            const result = await createActor().add_verification(payload);
            console.log('AddVerification', {result});

            return Promise.resolve(formatJSONResponse({
                statusCode: 200,
                body: {message: 'success', result},
            }));
        } catch (error) {
            console.error(`Error add_verification:  ${(error as Error).message}`);
            return Promise.reject(error);
        }
    });

    return Promise.allSettled(promises);
};

export const main = middy(publish).use(sqsJsonBodyParser()).use(sqsBatch());
