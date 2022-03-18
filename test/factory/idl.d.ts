import type { Principal } from '@dfinity/principal';
export interface Activity {
  'create_at' : string,
  'canister_id' : Principal,
  'build_status' : BuildStatus,
}
export interface BuildConfig {
  'updated_at' : string,
  'canister_id' : Principal,
  'created_at' : string,
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
  'addAdmin' : (arg_0: Principal) => Promise<undefined>,
  'addBuilder' : (arg_0: Principal) => Promise<undefined>,
  'addValidator' : (arg_0: Principal) => Promise<undefined>,
  'deleteAdmin' : (arg_0: Principal) => Promise<undefined>,
  'deleteBuildConfig' : (arg_0: Principal) => Promise<undefined>,
  'deleteBuilder' : (arg_0: Principal) => Promise<undefined>,
  'deleteValidator' : (arg_0: Principal) => Promise<undefined>,
  'getActivities' : (arg_0: PaginationInfo) => Promise<ManualReply>,
  'getAdmins' : () => Promise<Array<Principal>>,
  'getBuildConfigById' : (arg_0: Principal) => Promise<[] | [BuildConfig]>,
  'getBuildConfigValidator' : (arg_0: BuildConfigInfo) => Promise<
      [] | [BuildConfig]
    >,
  'getBuildConfigs' : () => Promise<Array<BuildConfig>>,
  'getBuilders' : () => Promise<Array<Principal>>,
  'getValidators' : () => Promise<Array<Principal>>,
  'getVerificationByCanisterId' : (arg_0: Principal) => Promise<
      [] | [Verification]
    >,
  'getVerifications' : (arg_0: PaginationInfo) => Promise<ManualReply_1>,
  'getVerificationsStats' : () => Promise<Stats>,
  'registerVerification' : (arg_0: RegisterVerification) => Promise<Result>,
  'saveBuildConfig' : (arg_0: SaveBuildConfig) => Promise<undefined>,
  'submitVerification' : (arg_0: SubmitVerification) => Promise<undefined>,
}
