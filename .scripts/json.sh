#!/bin/bash
# Add/change attributes in a json file using jq tool
# json.sh file.json key "Value"

#source "${BASH_SOURCE%/*}/utils.sh"
#(verifyExecDependency "jq")

OUTPUT_FILE=$1
if [[ ! -f $OUTPUT_FILE ]]; then
  echo "Creating new $OUTPUT_FILE"
  echo "{}" > $OUTPUT_FILE
fi

newJson=$(jq --arg key0 "$2" --arg value0 "$3" '.| .[$key0]=$value0' < $OUTPUT_FILE)
echo $newJson > $OUTPUT_FILE
echo $newJson
