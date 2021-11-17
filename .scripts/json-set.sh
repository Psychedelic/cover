#!/bin/bash
# Add/change attributes in a json file using jq tool
# json-set.sh file.json key "Value"

OUTPUT_FILE=$1
if [[ ! -f $OUTPUT_FILE ]]; then
  echo "Creating new $OUTPUT_FILE"
  echo "{}" > $OUTPUT_FILE
fi

newJson=$(jq --arg key0 "$2" --arg value0 "$3" '.| .[$key0]=$value0' < $OUTPUT_FILE)
echo $newJson > $OUTPUT_FILE

if [[ $DEBUG ]]; then
  echo $newJson
fi
