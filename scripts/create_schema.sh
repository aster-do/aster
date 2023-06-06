#!/bin/bash

if [ -n "$POSTGRESQL_SCHEMAS" ]; then
    #they are separated by commas
    IFS=',' read -ra ADDR <<< "$POSTGRESQL_SCHEMAS"
    for i in "${ADDR[@]}"; do
        echo "Creating schema $i"
        psql -h localhost -U postgres -c "CREATE SCHEMA $i;"
    done
fi