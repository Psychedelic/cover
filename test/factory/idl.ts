// @ts-nocheck
export const idlFactory = ({ IDL }) => {
  const Config = IDL.Record({
    'admin' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'validator' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'builder' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const PaginationInfo = IDL.Record({
    'page_index' : IDL.Nat64,
    'items_per_page' : IDL.Nat64,
  });
  const BuildStatus = IDL.Variant({
    'Error' : IDL.Null,
    'Building' : IDL.Null,
    'Success' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const Activity = IDL.Record({
    'create_at' : IDL.Text,
    'canister_id' : IDL.Principal,
    'build_status' : BuildStatus,
  });
  const ManualReply = IDL.Record({
    'page_index' : IDL.Nat64,
    'data' : IDL.Vec(Activity),
    'total_pages' : IDL.Nat64,
    'total_items' : IDL.Nat64,
    'is_first_page' : IDL.Bool,
    'items_per_page' : IDL.Nat64,
    'is_last_page' : IDL.Bool,
  });
  const BuildConfig = IDL.Record({
    'updated_at' : IDL.Text,
    'canister_id' : IDL.Principal,
    'created_at' : IDL.Text,
    'dfx_version' : IDL.Text,
    'owner_id' : IDL.Principal,
    'canister_name' : IDL.Text,
    'commit_hash' : IDL.Text,
    'repo_url' : IDL.Text,
    'rust_version' : IDL.Opt(IDL.Text),
    'optimize_count' : IDL.Nat8,
  });
  const BuildConfigInfo = IDL.Record({
    'canister_id' : IDL.Principal,
    'owner_id' : IDL.Principal,
  });
  const CanisterType = IDL.Variant({ 'Rust' : IDL.Null, 'Motoko' : IDL.Null });
  const Verification = IDL.Record({
    'updated_at' : IDL.Text,
    'updated_by' : IDL.Principal,
    'canister_id' : IDL.Principal,
    'dfx_version' : IDL.Text,
    'build_status' : BuildStatus,
    'canister_name' : IDL.Text,
    'commit_hash' : IDL.Text,
    'canister_type' : IDL.Opt(CanisterType),
    'repo_url' : IDL.Text,
    'repo_visibility' : IDL.Opt(IDL.Text),
    'rust_version' : IDL.Opt(IDL.Text),
    'optimize_count' : IDL.Nat8,
    'build_url' : IDL.Opt(IDL.Text),
    'wasm_hash' : IDL.Opt(IDL.Text),
  });
  const ManualReply_1 = IDL.Record({
    'page_index' : IDL.Nat64,
    'data' : IDL.Vec(Verification),
    'total_pages' : IDL.Nat64,
    'total_items' : IDL.Nat64,
    'is_first_page' : IDL.Bool,
    'items_per_page' : IDL.Nat64,
    'is_last_page' : IDL.Bool,
  });
  const Stats = IDL.Record({
    'build_error_count' : IDL.Nat64,
    'build_in_progress_count' : IDL.Nat64,
    'rust_canisters_count' : IDL.Nat64,
    'build_pending_count' : IDL.Nat64,
    'motoko_canisters_count' : IDL.Nat64,
    'total_canisters' : IDL.Nat64,
    'build_success_count' : IDL.Nat64,
  });
  const RegisterVerification = IDL.Record({
    'canister_id' : IDL.Principal,
    'dfx_version' : IDL.Text,
    'owner_id' : IDL.Principal,
    'canister_name' : IDL.Text,
    'commit_hash' : IDL.Text,
    'repo_url' : IDL.Text,
    'rust_version' : IDL.Opt(IDL.Text),
    'optimize_count' : IDL.Nat8,
  });
  const Error = IDL.Variant({ 'BuildInProgress' : IDL.Null });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const SaveBuildConfig = IDL.Record({
    'canister_id' : IDL.Principal,
    'dfx_version' : IDL.Text,
    'owner_id' : IDL.Principal,
    'canister_name' : IDL.Text,
    'commit_hash' : IDL.Text,
    'repo_url' : IDL.Text,
    'rust_version' : IDL.Opt(IDL.Text),
    'optimize_count' : IDL.Nat8,
  });
  const SubmitVerification = IDL.Record({
    'canister_id' : IDL.Principal,
    'dfx_version' : IDL.Text,
    'owner_id' : IDL.Principal,
    'build_status' : BuildStatus,
    'canister_name' : IDL.Text,
    'commit_hash' : IDL.Text,
    'canister_type' : IDL.Opt(CanisterType),
    'repo_url' : IDL.Text,
    'repo_visibility' : IDL.Opt(IDL.Text),
    'rust_version' : IDL.Opt(IDL.Text),
    'optimize_count' : IDL.Nat8,
    'build_url' : IDL.Text,
    'wasm_hash' : IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    'addAdmin' : IDL.Func([IDL.Principal], [], []),
    'addBuilder' : IDL.Func([IDL.Principal], [], []),
    'addValidator' : IDL.Func([IDL.Principal], [], []),
    'deleteAdmin' : IDL.Func([IDL.Principal], [], []),
    'deleteBuildConfig' : IDL.Func([IDL.Principal], [], []),
    'deleteBuilder' : IDL.Func([IDL.Principal], [], []),
    'deleteValidator' : IDL.Func([IDL.Principal], [], []),
    'getActivities' : IDL.Func([PaginationInfo], [ManualReply], ['query']),
    'getAdmins' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'getBuildConfigById' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(BuildConfig)],
        ['query'],
      ),
    'getBuildConfigValidator' : IDL.Func(
        [BuildConfigInfo],
        [IDL.Opt(BuildConfig)],
        ['query'],
      ),
    'getBuildConfigs' : IDL.Func([], [IDL.Vec(BuildConfig)], ['query']),
    'getBuilders' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'getValidators' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'getVerificationByCanisterId' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Verification)],
        ['query'],
      ),
    'getVerifications' : IDL.Func([PaginationInfo], [ManualReply_1], ['query']),
    'getVerificationsStats' : IDL.Func([], [Stats], ['query']),
    'registerVerification' : IDL.Func([RegisterVerification], [Result], []),
    'saveBuildConfig' : IDL.Func([SaveBuildConfig], [], []),
    'submitVerification' : IDL.Func([SubmitVerification], [], []),
  });
};
export const init = ({ IDL }) => {
  const Config = IDL.Record({
    'admin' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'validator' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'builder' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  return [IDL.Opt(Config)];
};
