import type { Principal } from '@dfinity/principal';
export interface AddProvider {
  'id' : Principal,
  'memo' : [] | [string],
  'name' : string,
}
export interface AddVerification {
  'wasm_checksum' : string,
  'source_snapshot_url' : string,
  'canister_id' : Principal,
  'git_checksum' : string,
  'canister_checksum' : string,
  'build_log_url' : string,
}
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
export interface Provider {
  'id' : Principal,
  'updated_at' : string,
  'updated_by' : Principal,
  'memo' : [] | [string],
  'name' : string,
  'created_at' : string,
  'created_by' : Principal,
}
export type ProviderInfo = {};
export interface Request {
  'request_id' : bigint,
  'canister_id' : Principal,
  'created_at' : string,
  'created_by' : Principal,
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
export interface UpdateProvider {
  'id' : Principal,
  'memo' : [] | [string],
  'name' : string,
}
export interface UpdateVerification {
  'wasm_checksum' : string,
  'source_snapshot_url' : string,
  'canister_id' : Principal,
  'git_checksum' : string,
  'canister_checksum' : string,
  'build_log_url' : string,
}
export interface Verification {
  'wasm_checksum' : string,
  'updated_at' : string,
  'updated_by' : Principal,
  'source_snapshot_url' : string,
  'canister_id' : Principal,
  'git_checksum' : string,
  'created_at' : string,
  'created_by' : Principal,
  'canister_checksum' : string,
  'build_log_url' : string,
}
export interface _SERVICE {
  'add_provider' : (arg_0: AddProvider) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
  'add_verification' : (arg_0: AddVerification) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
  'delete_provider' : (arg_0: Principal) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
  'get_all_providers' : () => Promise<Array<Provider>>,
  'get_all_verifications' : () => Promise<Array<Verification>>,
  'get_provider_by_id' : (arg_0: Principal) => Promise<[] | [Provider]>,
  'get_verification_by_canister_id' : (arg_0: Principal) => Promise<
      [] | [Verification]
    >,
  'update_provider' : (arg_0: UpdateProvider) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
  'update_verification' : (arg_0: UpdateVerification) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
}
