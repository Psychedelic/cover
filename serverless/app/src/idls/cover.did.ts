export const idlFactory = ({ IDL }) => {
  const AddProvider = IDL.Record({
    'id' : IDL.Principal,
    'memo' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
  });
  const Error = IDL.Record({
    'debug_log' : IDL.Opt(IDL.Text),
    'code' : IDL.Text,
    'message' : IDL.Text,
  });
  const AddVerification = IDL.Record({
    'wasm_checksum' : IDL.Text,
    'source_snapshot_url' : IDL.Text,
    'canister_id' : IDL.Principal,
    'git_repo' : IDL.Text,
    'git_ref' : IDL.Text,
    'git_sha' : IDL.Text,
    'build_log_url' : IDL.Text,
  });
  const Provider = IDL.Record({
    'id' : IDL.Principal,
    'updated_at' : IDL.Text,
    'updated_by' : IDL.Principal,
    'memo' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'created_at' : IDL.Text,
    'created_by' : IDL.Principal,
  });
  const Verification = IDL.Record({
    'wasm_checksum' : IDL.Text,
    'updated_at' : IDL.Text,
    'updated_by' : IDL.Principal,
    'source_snapshot_url' : IDL.Text,
    'canister_id' : IDL.Principal,
    'created_at' : IDL.Text,
    'created_by' : IDL.Principal,
    'git_repo' : IDL.Text,
    'git_ref' : IDL.Text,
    'git_sha' : IDL.Text,
    'build_log_url' : IDL.Text,
  });
  const UpdateProvider = IDL.Record({
    'id' : IDL.Principal,
    'memo' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
  });
  const UpdateVerification = IDL.Record({
    'wasm_checksum' : IDL.Text,
    'source_snapshot_url' : IDL.Text,
    'canister_id' : IDL.Principal,
    'git_repo' : IDL.Text,
    'git_ref' : IDL.Text,
    'git_sha' : IDL.Text,
    'build_log_url' : IDL.Text,
  });
  return IDL.Service({
    'add_provider' : IDL.Func(
        [AddProvider],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
    'add_verification' : IDL.Func(
        [AddVerification],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
    'delete_provider' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
    'get_all_providers' : IDL.Func([], [IDL.Vec(Provider)], []),
    'get_all_verifications' : IDL.Func([], [IDL.Vec(Verification)], []),
    'get_provider_by_id' : IDL.Func([IDL.Principal], [IDL.Opt(Provider)], []),
    'get_verification_by_canister_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Verification)],
        [],
      ),
    'update_provider' : IDL.Func(
        [UpdateProvider],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
    'update_verification' : IDL.Func(
        [UpdateVerification],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
