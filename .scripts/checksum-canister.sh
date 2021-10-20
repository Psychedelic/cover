#!/bin/bash
CANISTER_ID=$1
resp=$(dfx canister --no-wallet --network ic info $CANISTER_ID)
# Returns
# > Controller: r7inp-6aaaa-aaaaa-aaabq-cai
# > Module hash: 0x83ad30e332119989e5f7cd67a69d176349f50a6c5d2591c9786930d6b57cdabe
echo $resp | awk '/hash: (.+)$/{print $5}'
