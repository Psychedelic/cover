#!/bin/bash
# Fetch request from cover canister,  store it in local file system and symlink as fetched.json
# If no file is received, remove fetched.json

echo "Fetching request validation ..."
RESPONSE=$(.scripts/call-to-json.sh fetch_validation_json)

timestamp=$(echo '('$(date +"%s.%N") ') * 100 / 1'|bc)
FILENAME="fetched-$timestamp.json"
TARGET_DIR="./tmp/cover-requests"
TARGET_JSON="$TARGET_DIR/$FILENAME"
TARGET_SYMLINK="$TARGET_DIR/fetched.json"

mkdir -p $TARGET_DIR
rm -f $TARGET_SYMLINK

if [[ $RESPONSE ]]; then
  echo $RESPONSE > $TARGET_JSON
  cd $TARGET_DIR; ln -s $FILENAME fetched.json; cd -
  echo "Saved new request: $TARGET_SYMLINK -> $TARGET_JSON"
else
  echo "No new requests found!"
fi
