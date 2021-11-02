#!/bin/bash
source "${BASH_SOURCE%/*}/utils.sh"

JSON_PATH=$1

debug() {
  JSON=$(jq < $JSON_PATH)
  echo "================================"
  echo "Settings File: $JSON_PATH"
  echo "Content: $JSON"
  echo "================================"
}

if [[ $DEBUG ]]; then (debug); fi

echo "CAN_ID: $(jget canister_id)"
echo "GIT_REF: $(jget build_settings.git_ref)"
jset called_at "$((231 * 44))"
echo "Called at $(jget called_at)"
