#!/bin/bash

source "${BASH_SOURCE%/*}/utils.sh"

verifyDependency CANISTER_NAME

echo "🙏 Verifying the service status, please wait..."

dfx canister status "$CANISTER_NAME"

status=$?

if [ $status -ne 0 ]; then
  echo "🤖 Oops! The canister $CANISTER_NAME is not running..."

  exit 1;
fi;

echo "🤖 The canister $CANISTER_NAME service is running"
echo ""
