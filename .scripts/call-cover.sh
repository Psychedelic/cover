#!/bin/bash
source "${BASH_SOURCE%/*}/utils.sh"
verifyExecDependency jq
verifyDependency DFX_NETWORK

NAME=$1
JSON=$(jq tojson $2)
echo "Calling $DFX_NETWORK: cover.$NAME($JSON)"
dfx canister --network=$DFX_NETWORK call cover $NAME "$JSON"
