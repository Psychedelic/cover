#!/bin/bash
# Get an attribute value from a json file using jq tool
# json-get.sh file.json attributeName

INPUT_FILE=$1
FIELD=$2

if [[ ! -f $INPUT_FILE ]]; then
  echo "File not found $INPUT_FILE"
  exit 1;
fi

cat $INPUT_FILE | jq ".[\"$FIELD\"]"
