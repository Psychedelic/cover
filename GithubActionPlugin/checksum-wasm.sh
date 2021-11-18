#!/bin/bash
FILE=$1
if [[ -f $FILE ]]; then
  openssl dgst -sha256 $FILE| awk '/.+$/{print "0x"$2}'
else
  echo "NOTFOUND:$FILE"
  exit 1;
fi
