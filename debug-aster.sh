#! /bin/bash

# This script is used to debug asterisk

docker-compose up -d
export DATABASE_URL=postgres://postgres:postgres@localhost:5432
cargo  run -p aster