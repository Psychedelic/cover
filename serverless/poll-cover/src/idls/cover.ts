export const idlFactory = ({ IDL }) => {
  const ProviderInfo = IDL.Record({});
  const BuildSettings = IDL.Record({
    'git_ref' : IDL.Text,
    'git_tag' : IDL.Text,
  });
  const Request = IDL.Record({
    'request_id' : IDL.Nat64,
    'canister_id' : IDL.Principal,
    'created_at' : IDL.Text,
    'caller_id' : IDL.Principal,
    'build_settings' : BuildSettings,
  });
  const Error = IDL.Record({
    'debug_log' : IDL.Opt(IDL.Text),
    'code' : IDL.Text,
    'message' : IDL.Text,
  });
  const CreateRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'build_settings' : BuildSettings,
  });
  const ProgressStatus = IDL.Variant({
    'Error' : IDL.Null,
    'Init' : IDL.Null,
    'Finished' : IDL.Null,
    'InProgress' : IDL.Null,
  });
  const Progress = IDL.Record({
    'request_id' : IDL.Nat64,
    'status' : ProgressStatus,
    'wasm_checksum' : IDL.Opt(IDL.Text),
    'updated_at' : IDL.Opt(IDL.Text),
    'source_snapshot_url' : IDL.Opt(IDL.Text),
    'canister_id' : IDL.Principal,
    'git_checksum' : IDL.Opt(IDL.Text),
    'canister_checksum' : IDL.Opt(IDL.Text),
    'build_log_url' : IDL.Opt(IDL.Text),
    'percentage' : IDL.Opt(IDL.Float32),
    'started_at' : IDL.Text,
  });
  const UpdateProgress = IDL.Record({
    'request_id' : IDL.Nat64,
    'status' : ProgressStatus,
    'wasm_checksum' : IDL.Opt(IDL.Text),
    'source_snapshot_url' : IDL.Opt(IDL.Text),
    'canister_id' : IDL.Principal,
    'git_checksum' : IDL.Opt(IDL.Text),
    'canister_checksum' : IDL.Opt(IDL.Text),
    'build_log_url' : IDL.Opt(IDL.Text),
    'percentage' : IDL.Opt(IDL.Float32),
  });
  return IDL.Service({
    'consume_request' : IDL.Func(
        [ProviderInfo],
        [IDL.Variant({ 'Ok' : IDL.Vec(Request), 'Err' : Error })],
        [],
      ),
    'create_request' : IDL.Func(
        [CreateRequest],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
    'get_all_progress' : IDL.Func([], [IDL.Vec(Progress)], []),
    'get_all_request' : IDL.Func([], [IDL.Vec(Request)], []),
    'get_progress_by_canister_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(Progress)],
        [],
      ),
    'get_progress_by_request_id' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(Progress)],
        [],
      ),
    'get_request_by_id' : IDL.Func([IDL.Nat64], [IDL.Opt(Request)], []),
    'update_progress' : IDL.Func(
        [UpdateProgress],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
