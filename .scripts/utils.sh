#!/bin/bash

verifyDependency() {
  for var in "$@"; do
    if [[ ! ${!var+set} ]]; then
      echo "🤡 Oops! Missing the $var environment variable..."

      exit 1
    fi;
  done;
};

verifyExecDependency() {
  for cmd in "$@"; do
    echo "❓ Verifying $cmd CLI dependency..."
    if ! command -v "$cmd" &> /dev/null; then
        echo "🤡 Oops! Missing $cmd CLI, install and add to your path as <$cmd>"
        exit 1
    fi
    echo "👍 $cmd CLI is available"
  done;
}

jget() {
  .scripts/json-get.sh $JSON_PATH $1
}
jset() {
  .scripts/json-set.sh $JSON_PATH $1 $2
}
