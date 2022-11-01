import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Activity {
  'canister_id' : Principal,
  'created_at' : bigint,
  'build_status' : BuildStatus,
}
export interface BuildConfig {
  'updated_at' : bigint,
  'canister_id' : Principal,
  'caller_id' : Principal,
  'delegate_canister_id' : [] | [Principal],
  'dfx_version' : string,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export interface BuildConfigInfo {
  'canister_id' : Principal,
  'caller_id' : Principal,
}
export type BuildStatus = { 'Error' : null } |
  { 'Building' : null } |
  { 'Success' : null } |
  { 'Pending' : null };
export type CanisterType = { 'Rust' : null } |
  { 'Custom' : null } |
  { 'Motoko' : null };
export interface Config {
  'admin' : [] | [Array<Principal>],
  'validator' : [] | [Array<Principal>],
  'builder' : [] | [Array<Principal>],
}
export interface CoverMetadata {
  'dfx_version' : string,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export type Error = { 'BuildInProgress' : null };
export interface ManualReply {
  'page_index' : bigint,
  'data' : Array<Activity>,
  'total_pages' : bigint,
  'total_items' : bigint,
  'is_first_page' : boolean,
  'items_per_page' : bigint,
  'is_last_page' : boolean,
}
export interface ManualReply_1 {
  'page_index' : bigint,
  'data' : Array<Verification>,
  'total_pages' : bigint,
  'total_items' : bigint,
  'is_first_page' : boolean,
  'items_per_page' : bigint,
  'is_last_page' : boolean,
}
export interface PaginationInfo {
  'page_index' : bigint,
  'items_per_page' : bigint,
}
export interface RegisterVerification {
  'canister_id' : Principal,
  'caller_id' : Principal,
  'delegate_canister_id' : [] | [Principal],
  'dfx_version' : string,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'repo_visibility' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export interface SaveBuildConfig {
  'canister_id' : Principal,
  'caller_id' : Principal,
  'delegate_canister_id' : [] | [Principal],
  'dfx_version' : string,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export interface Stats {
  'custom_canisters_count' : bigint,
  'build_error_count' : bigint,
  'build_in_progress_count' : bigint,
  'rust_canisters_count' : bigint,
  'build_pending_count' : bigint,
  'motoko_canisters_count' : bigint,
  'total_canisters' : bigint,
  'build_success_count' : bigint,
}
export interface SubmitVerification {
  'canister_id' : Principal,
  'caller_id' : Principal,
  'delegate_canister_id' : [] | [Principal],
  'dfx_version' : string,
  'build_status' : BuildStatus,
  'canister_name' : string,
  'commit_hash' : string,
  'canister_type' : [] | [CanisterType],
  'repo_url' : string,
  'repo_visibility' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
  'build_url' : string,
  'wasm_hash' : [] | [string],
}
export interface Verification {
  'updated_at' : bigint,
  'updated_by' : Principal,
  'canister_id' : Principal,
  'delegate_canister_id' : [] | [Principal],
  'dfx_version' : string,
  'build_status' : BuildStatus,
  'canister_name' : string,
  'commit_hash' : string,
  'canister_type' : [] | [CanisterType],
  'repo_url' : string,
  'repo_visibility' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
  'build_url' : [] | [string],
  'wasm_hash' : [] | [string],
}
export interface _SERVICE {
  'addAdmin' : ActorMethod<[Principal], undefined>,
  'addBuilder' : ActorMethod<[Principal], undefined>,
  'addValidator' : ActorMethod<[Principal], undefined>,
  'coverMetadata' : ActorMethod<[], CoverMetadata>,
  'deleteAdmin' : ActorMethod<[Principal], undefined>,
  'deleteBuildConfig' : ActorMethod<[Principal], undefined>,
  'deleteBuilder' : ActorMethod<[Principal], undefined>,
  'deleteValidator' : ActorMethod<[Principal], undefined>,
  'getActivities' : ActorMethod<[PaginationInfo], ManualReply>,
  'getAdmins' : ActorMethod<[], Array<Principal>>,
  'getBuildConfigById' : ActorMethod<[Principal], [] | [BuildConfig]>,
  'getBuildConfigValidator' : ActorMethod<
    [BuildConfigInfo],
    [] | [BuildConfig],
  >,
  'getBuildConfigs' : ActorMethod<[], Array<BuildConfig>>,
  'getBuilders' : ActorMethod<[], Array<Principal>>,
  'getMyActivities' : ActorMethod<[PaginationInfo], ManualReply>,
  'getValidators' : ActorMethod<[], Array<Principal>>,
  'getVerificationByCanisterId' : ActorMethod<[Principal], [] | [Verification]>,
  'getVerifications' : ActorMethod<[PaginationInfo], ManualReply_1>,
  'getVerificationsStats' : ActorMethod<[], Stats>,
  'registerVerification' : ActorMethod<[RegisterVerification], Result>,
  'saveBuildConfig' : ActorMethod<[SaveBuildConfig], undefined>,
  'submitVerification' : ActorMethod<[SubmitVerification], undefined>,
}
