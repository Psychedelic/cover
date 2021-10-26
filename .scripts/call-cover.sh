#!/bin/bash
NAME=$1
JSON_FILE=$2
if [[ ! -f $JSON_FILE ]]; then
  JSON=""
else
  JSON="$(jq tojson "$JSON_FILE")"
fi;
echo "Calling $DFX_NETWORK: cover.$NAME($JSON)"
dfx canister --network=$DFX_NETWORK call cover $NAME $JSON
