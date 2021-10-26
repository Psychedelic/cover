#!/bin/bash
# call cover canister, extract json from response
RESPONSE=$(.scripts/call-cover.sh $@)
# remove (" ") from the response
JSON=$(echo "$RESPONSE" | awk '/"{/{flag=1; next} /}"/{flag=0} flag')
echo "{ $JSON }" | jq
