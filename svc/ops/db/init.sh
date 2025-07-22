#!/usr/bin/env bash
##set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: `psql` command is not available (not installed or its location is not in the PATH)."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 ""
  echo >&2 ">>> Error: `sqlx` command is not available (not installed or its location is not in the PATH)."
  echo >&2 ">>> Hint: `sqlx-cli` needs to be installed. See ./readme.md for instructions."
  echo >&2 ""
  exit 1
fi

DB_IMAGE="postgres:17-alpine"

# Check if a custom user has been set, otherwise default to 'cogs'.
DB_USER="${POSTGRES_USER:=cogs}"
# Check if a custom password has been set, otherwise default to 'cogs'.
DB_PASSWORD="${POSTGRES_PASSWORD:=cogs}"
# Check if a custom password has been set, otherwise default to 'cogs'.
DB_NAME="${POSTGRES_DB:=cogs}"
# Check if a custom port has been set, otherwise default to '5443'.
DB_PORT="${POSTGRES_PORT:=5451}"
# Check if a custom host has been set, otherwise default to 'localhost'.
DB_HOST="${POSTGRES_HOST:=localhost}"

# Allow to skip Docker if a dockerized PostgreSQL database is already running.
if [[ -z "${SKIP_DOCKER}" ]]
then
  # if a PostgreSQL container is running, print instructions to kill it and exit.
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=cogs' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 ">>> There is a container named 'cogs' already running."
    echo >&2 ">>> Terminate it with `docker kill ${RUNNING_POSTGRES_CONTAINER}`"
    exit 1
  fi
  # Launch PostgreSQL container.
  docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d \
      --name "cogs_$(date '+%Y%m%d_%H%M%S')" \
      ${DB_IMAGE} -N 200
      # ^ Increased the maximum number of connections for testing purposes.
fi

echo ">>> Waiting for PostgreSQL to start ..."
sleep 4

# Keep polling PostgreSQL instance for its readiness.
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo ">>> PostgreSQL is still unavailable. Sleeping ..."
  sleep 1
done

echo ">>> PostgreSQL instance is up and running on port ${DB_PORT}."
echo ">>> Running the database migrations ..."

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create

BASEDIR=$(dirname $0)
sqlx migrate run --source $BASEDIR/migrations

echo ">>> Completed the database migrations."
