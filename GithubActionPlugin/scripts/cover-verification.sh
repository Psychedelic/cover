#!/bin/bash
CANISTER_ID=$1
if [[ -z $2 ]]; then
  NETWORK=local
else
  NETWORK=$2
fi
resp=$(
dfx canister --network=$NETWORK call --output=idl cover get_verification_by_canister_id "(principal\"$CANISTER_ID\")"
)
echo $resp | awk '/wasm_checksum = "(.+?)"/{print $0}'
