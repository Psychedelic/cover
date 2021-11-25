import { FromSchema } from 'json-schema-to-ts';

export interface CoverPayloadI {
  access_token: string;
  canister_id: string;
  created_at: string;
  git_ref: string;
  git_sha: string;
  git_repo: string;
  wasm_path: string;
  wasm_checksum: string;
  source_snapshot_url: string;
  build_log_url: string;
}

export const CoverSchema = {
  type: 'object',
  properties: {
    access_token: {
      type: 'string',
    },
    canister_id: {
      type: 'string',
    },
    created_at: {
      type: 'string',
    },
    git_ref: {
      type: 'string',
    },
    git_repo: {
      type: 'string',
    },
    git_sha: {
      type: 'string',
    },
    wasm_path: {
      type: 'string',
    },
    wasm_checksum: {
      type: 'string',
    },
    build_log_url: {
      type: 'string',
      format: 'uri',
    },
    source_snapshot_url: {
      type: 'string',
    },
  },
  required: ['url', 'serviceName'],
} as const;

export type CoverPayload = FromSchema<typeof CoverSchema>;
