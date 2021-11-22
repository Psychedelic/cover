#!/bin/bash
echo "Creating docker builder dfx-builder"
docker build -t fleek/dfx-builder -f .docker/deployment/Dockerfile .
echo "Build docker fleek/dfx-builder completed ..."
echo "-----------------------------------------------------"
TARGET="$(pwd)/dfx-build"
echo "Cleaning target dir: $TARGET"
rm -rf $TARGET
echo "Starting builder, output container /workspace/.dfx -> $TARGET"
echo "Executing build script ${BASH_SOURCE%/*}/entrypoint.sh"
echo "-----------------------------------------------------"
cat ${BASH_SOURCE%/*}/entrypoint.sh
echo "-----------------------------------------------------"
RUN="docker run --rm -ti -v "$TARGET":"/workspace/.dfx" fleek/dfx-builder"
echo "Running: $RUN"
($RUN)
echo "Build completed in $TARGET"
