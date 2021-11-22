#!/bin/bash
set +x
yarn
dfx build --all --check
tree .dfx
