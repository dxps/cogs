#!/bin/sh

BASEDIR=$(dirname $0)
CHANGENAME=$1

sqlx migrate add $CHANGENAME --source $BASEDIR/migrations
