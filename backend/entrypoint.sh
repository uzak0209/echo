#!/bin/bash
set -e

echo "Waiting for postgres to be ready..."
until PGPASSWORD=password psql -h postgres -U postgres -d echo -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

echo "PostgreSQL is up - running migrations"
cd /app/migration && cargo run --release -- up

echo "Starting application with cargo watch"
cd /app
exec cargo watch -x run
