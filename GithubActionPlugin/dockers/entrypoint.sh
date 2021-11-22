#!/bin/bash
set +x
yarn
MODE=PRODUCTION dfx build cover --check
tree .dfx
