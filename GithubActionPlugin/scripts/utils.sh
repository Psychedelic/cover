#!/bin/bash

verifyDependency() {
  for var in "$@"; do
    if [[ ! -v ${var} ]]; then
      echo "🤡 Oops! Missing the $var environment variable..."
      exit 1
    fi
  done
}

jget() {
  verifyDependency JSON_PATH
  ${BASH_SOURCE%/*}/json-get.sh $JSON_PATH $1
}

jset() {
  verifyDependency JSON_PATH
  ${BASH_SOURCE%/*}/json-set.sh $JSON_PATH $1 $2
}

wasm_checksum() {
  ${BASH_SOURCE%/*}/checksum-wasm.sh $1
}

canister_checksum() {
  ${BASH_SOURCE%/*}/checksum-canister.sh $1
}

timestamp() {
  date +"%Y/%m/%d_%H:%M:%S:%N"
}

