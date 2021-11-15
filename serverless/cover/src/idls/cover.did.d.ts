import type { Principal } from '@dfinity/principal';
export interface BuildSettings { 'git_ref' : string, 'git_tag' : string }
export interface CreateRequest {
  'canister_id' : Principal,
  'build_settings' : BuildSettings,
}
export interface Error {
  'debug_log' : [] | [string],
  'code' : string,
  'message' : string,
}
export interface Progress {
  'request_id' : bigint,
  'status' : ProgressStatus,
  'wasm_checksum' : [] | [string],
  'updated_at' : [] | [string],
  'source_snapshot_url' : [] | [string],
  'canister_id' : Principal,
  'git_checksum' : [] | [string],
  'canister_checksum' : [] | [string],
  'build_log_url' : [] | [string],
  'percentage' : [] | [number],
  'started_at' : string,
}
export type ProgressStatus = { 'Error' : null } |
  { 'Init' : null } |
  { 'Finished' : null } |
  { 'InProgress' : null };
export type ProviderInfo = {};
export interface Request {
  'request_id' : bigint,
  'canister_id' : Principal,
  'created_at' : string,
  'caller_id' : Principal,
  'build_settings' : BuildSettings,
}
export interface UpdateProgress {
  'request_id' : bigint,
  'status' : ProgressStatus,
  'wasm_checksum' : [] | [string],
  'source_snapshot_url' : [] | [string],
  'canister_id' : Principal,
  'git_checksum' : [] | [string],
  'canister_checksum' : [] | [string],
  'build_log_url' : [] | [string],
  'percentage' : [] | [number],
}
export interface _SERVICE {
  'consume_request' : (arg_0: ProviderInfo) => Promise<
      { 'Ok' : Array<Request> } |
        { 'Err' : Error }
    >,
  'create_request' : (arg_0: CreateRequest) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
  'get_all_progress' : () => Promise<Array<Progress>>,
  'get_all_request' : () => Promise<Array<Request>>,
  'get_progress_by_canister_id' : (arg_0: Principal) => Promise<
      Array<Progress>
    >,
  'get_progress_by_request_id' : (arg_0: bigint) => Promise<[] | [Progress]>,
  'get_request_by_id' : (arg_0: bigint) => Promise<[] | [Request]>,
  'update_progress' : (arg_0: UpdateProgress) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
}
