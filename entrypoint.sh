#!/bin/sh
echo $DATABASE
if [ "$DATABASE" = "postgres" ]
then
  echo "Waiting for postgres..."

  while ! nc -z $SQL_HOST $SQL_PORT; do
    sleep 2
  done

  echo "PostgreSQL started"
  ./user_service
  # run migrations
fi

exec "$@"
