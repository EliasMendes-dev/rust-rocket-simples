#!/bin/bash

DB_HOST="localhost"
DB_USER="root"
DB_PASS="root"

echo "Criando banco de dados..."

mysql \
    --default-character-set=utf8mb4 \
    -h "$DB_HOST" \
    -u "$DB_USER" \
    -p"$DB_PASS" \
    < database/init.sql

echo "Banco criado com sucesso!"