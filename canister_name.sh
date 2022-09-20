#!/bin/bash
if [[ $(git branch --show-current) = 'main' ]]
then
	echo 'cover'
else
	echo 'cover_test'
fi
