import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Activity {
  'create_at' : string,
  'canister_id' : Principal,
  'build_status' : BuildStatus,
}
export interface BuildConfig {
  'updated_at' : string,
  'canister_id' : Principal,
  'dfx_version' : string,
  'owner_id' : Principal,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export interface BuildConfigInfo {
  'canister_id' : Principal,
  'owner_id' : Principal,
}
export type BuildStatus = { 'Error' : null } |
  { 'Building' : null } |
  { 'Success' : null } |
  { 'Pending' : null };
export type CanisterType = { 'Rust' : null } |
  { 'Motoko' : null };
export interface Config {
  'admin' : [] | [Array<Principal>],
  'validator' : [] | [Array<Principal>],
  'builder' : [] | [Array<Principal>],
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
  'dfx_version' : string,
  'owner_id' : Principal,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export interface SaveBuildConfig {
  'canister_id' : Principal,
  'dfx_version' : string,
  'owner_id' : Principal,
  'canister_name' : string,
  'commit_hash' : string,
  'repo_url' : string,
  'rust_version' : [] | [string],
  'optimize_count' : number,
}
export interface Stats {
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
  'dfx_version' : string,
  'owner_id' : Principal,
  'build_status' : BuildStatus,
  'canister_name' : string,
  'commit_hash' : string,
  'canister_type' : [] | [CanisterType],
  'repo_url' : string,
  'repo_visibility' : [] | [string],
  'rust_version' : [] | [string],
  'optimize_count' : number,
  'build_url' : string,
  'wasm_hash' : [] | [string],
}
export interface Verification {
  'updated_at' : string,
  'updated_by' : Principal,
  'canister_id' : Principal,
  'dfx_version' : string,
  'build_status' : BuildStatus,
  'canister_name' : string,
  'commit_hash' : string,
  'canister_type' : [] | [CanisterType],
  'repo_url' : string,
  'repo_visibility' : [] | [string],
  'rust_version' : [] | [string],
  'optimize_count' : number,
  'build_url' : [] | [string],
  'wasm_hash' : [] | [string],
}
export interface _SERVICE {
  'addAdmin' : ActorMethod<[Principal], undefined>,
  'addBuilder' : ActorMethod<[Principal], undefined>,
  'addValidator' : ActorMethod<[Principal], undefined>,
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
  'getValidators' : ActorMethod<[], Array<Principal>>,
  'getVerificationByCanisterId' : ActorMethod<[Principal], [] | [Verification]>,
  'getVerifications' : ActorMethod<[PaginationInfo], ManualReply_1>,
  'getVerificationsStats' : ActorMethod<[], Stats>,
  'registerVerification' : ActorMethod<[RegisterVerification], Result>,
  'saveBuildConfig' : ActorMethod<[SaveBuildConfig], undefined>,
  'submitVerification' : ActorMethod<[SubmitVerification], undefined>,
}
