#!/bin/bash

verifyDependency() {
  for var in "$@"; do
    if [[ ! -v ${var} ]]; then
      echo "🤡 Oops! Missing the $var environment variable..."
      exit 1
    fi
  done
}

verifyExecDependency() {
  for cmd in "$@"; do
    echo "❓ Verifying $cmd CLI dependency..."
    if ! command -v "$cmd" &> /dev/null; then
        echo "🤡 Oops! Missing $cmd CLI, install and add to your path as <$cmd>"
        exit 1
    fi
    echo "👍 $cmd CLI is available"
  done
}

jget() {
  verifyDependency JSON_PATH
  .scripts/json-get.sh $JSON_PATH $1
}
jset() {
  verifyDependency JSON_PATH
  .scripts/json-set.sh $JSON_PATH $1 $2
}
wasm_checksum() {
  .scripts/checksum-wasm.sh $1
}


timestamp() {
  date +"%Y/%m/%d_%H:%M:%S:%N"
}

