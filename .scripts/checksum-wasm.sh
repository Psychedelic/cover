#!/bin/bash
FILE=$1
openssl dgst -sha256 $FILE| awk '/.+$/{print "0x"$2}'
