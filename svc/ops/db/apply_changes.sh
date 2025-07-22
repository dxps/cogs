#!/bin/sh

BASEDIR=$(dirname $0)

DATABASE_URL=postgresql://cogs:cogs@localhost:5451/cogs

sqlx migrate run --source $BASEDIR/migrations --database-url $DATABASE_URL
