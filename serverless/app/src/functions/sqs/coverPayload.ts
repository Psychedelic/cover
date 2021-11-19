import { FromSchema } from 'json-schema-to-ts';

export interface CoverPayloadI {
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
    canister_id: {
      type: 'string',
    },
    created_at: {
      type: 'string',
    },
    git_ref: {
      type: 'string',
    },
    source_snapshot_url: {
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
  },
  required: ['url', 'serviceName'],
} as const;

export type CoverPayload = FromSchema<typeof CoverSchema>;
