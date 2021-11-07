export const idlFactory = ({ IDL }) => {
  const BuildParams = IDL.Record({
    'git_ref' : IDL.Text,
    'git_tag' : IDL.Text,
  });
  const NewValidationRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'validator_id' : IDL.Opt(IDL.Principal),
    'build_settings' : BuildParams,
  });
  const Error = IDL.Record({
    'debug_log' : IDL.Opt(IDL.Text),
    'code' : IDL.Text,
    'message' : IDL.Text,
  });
  const ProviderInfo = IDL.Record({});
  const ValidationRequest = IDL.Record({
    'request_id' : IDL.Nat64,
    'canister_id' : IDL.Principal,
    'caller_id' : IDL.Principal,
    'build_settings' : BuildParams,
  });
  return IDL.Service({
    'add_request' : IDL.Func(
        [NewValidationRequest],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
    ),
    'consume_request' : IDL.Func(
        [ProviderInfo],
        [IDL.Variant({ 'Ok' : IDL.Vec(ValidationRequest), 'Err' : Error })],
        [],
    ),
    'get_all_pending_request' : IDL.Func([], [IDL.Vec(ValidationRequest)], []),
    'get_pending_request_by_id' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(ValidationRequest)],
        [],
    ),
    'whoami' : IDL.Func([], [IDL.Principal], []),
  });
};
export const init = ({ IDL }) => { return []; };
