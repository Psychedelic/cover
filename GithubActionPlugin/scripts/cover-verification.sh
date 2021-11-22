#!/bin/bash
CANISTER_ID=$1
if [[ -z $2 ]]; then
  NETWORK=ic
else
  NETWORK=$2
fi
resp=$(
dfx canister --network=$NETWORK call --output=idl cover get_verification_by_canister_id "(principal\"$CANISTER_ID\")"
)
echo "Fetching verifier of $CANISTER_ID on $NETWORK"
wasm=$(echo $resp |sed  -e 's/[;{]/\n\r/g'|sed -e 's/"//g'|grep wasm|awk '/ = (.+)$/{print $4}')
echo "Fetching canister checksum $CANISTER_ID on $NETWORK"
resp=$(dfx canister --no-wallet --network $NETWORK info $CANISTER_ID)
cani=$(echo $resp |sed  -e 's/Module/\n\r/g'| grep hash| awk '/hash: (.+)$/{print $3}')


echo "Wasm checksum: $wasm"
echo "Module hash: $cani"
if [[ $wasm == $cani && $wasm ]]; then
  echo "Status: Verified"
  exit 0
else
  echo "Status: Unverified"
  exit 1
fi
