#!/bin/bash
set +x
yarn
MODE=PRODUCTION dfx build --all --check
tree .dfx
