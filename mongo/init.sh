#!/usr/bin/env bash
echo "Creating mongo users..."
mongosh --authenticationDatabase admin \
--host localhost \
-u $MONGO_INITDB_ROOT_USERNAME \
-p $MONGO_INITDB_ROOT_PASSWORD \
$MONGO_APP_DB \
--eval "db.createUser({user: '$MONGO_APP_USER', pwd: '$MONGO_APP_PASS', roles: [{role: 'readWrite', db: '$MONGO_APP_DB'}]});"
echo "Mongo users created."
