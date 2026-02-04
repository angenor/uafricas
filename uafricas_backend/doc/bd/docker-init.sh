#!/bin/bash
set -e

echo "=== Initialisation du schéma africans_db ==="

cd /docker-entrypoint-initdb.d/bd
psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f schema.sql

echo "=== Schéma initialisé avec succès ==="
