import type { Principal } from '@dfinity/principal';
export interface BuildParams { 'git_ref' : string, 'git_tag' : string }
export interface Error {
  'debug_log' : [] | [string],
  'code' : string,
  'message' : string,
}
export interface NewValidationRequest {
  'canister_id' : Principal,
  'validator_id' : [] | [Principal],
  'build_settings' : BuildParams,
}
export type ProviderInfo = {};
export interface ValidationRequest {
  'request_id' : bigint,
  'canister_id' : Principal,
  'caller_id' : Principal,
  'build_settings' : BuildParams,
}
export interface _SERVICE {
  'add_request' : (arg_0: NewValidationRequest) => Promise<
      { 'Ok' : null } |
        { 'Err' : Error }
    >,
  'consume_request' : (arg_0: ProviderInfo) => Promise<
      { 'Ok' : Array<ValidationRequest> } |
        { 'Err' : Error }
    >,
  'get_all_pending_request' : () => Promise<Array<ValidationRequest>>,
  'get_pending_request_by_id' : (arg_0: bigint) => Promise<
      [] | [ValidationRequest]
    >,
  'whoami' : () => Promise<Principal>,
}
