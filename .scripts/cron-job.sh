#!/bin/bash

MULTI=0
doubleTime () {
  local -n M=$1
  if (( $M < 1)); then
    M=1
  elif (( ($M) < 10 )); then
    M=$((M*2))
  fi
}

while :
do
  .scripts/fetch-request.sh
  STATUS=$?

  if [ $STATUS -ne 0 ]; then
    doubleTime MULTI
    WAIT=$((MULTI * 2))
    echo "Waiting $WAIT secs"
    sleep $WAIT
  else
    MULTI=0 # reset time multiplier
    .scripts/build-job.sh $PWD/tmp/cover-requests/fetched.json
  fi
done
