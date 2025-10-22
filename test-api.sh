#!/bin/bash

# Script para probar la API Rust con Actix-web

BASE_URL="http://localhost:3000"

echo "🚀 Probando API Rust con Actix-web"
echo "==================================="

# Health check
echo -e "\n1. Health Check:"
curl -s "$BASE_URL/health" | jq .

# API info
echo -e "\n2. API Information:"
curl -s "$BASE_URL/api" | jq .

# Create user
echo -e "\n3. Creating user:"
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{"name": "Juan Pérez", "email": "juan@example.com"}')
echo "$USER_RESPONSE" | jq .

# Extract user ID
USER_ID=$(echo "$USER_RESPONSE" | jq -r '.data.id')

if [ "$USER_ID" != "null" ]; then
  # Get user
  echo -e "\n4. Getting user by ID ($USER_ID):"
  curl -s "$BASE_URL/api/users/$USER_ID" | jq .

  # Update user
  echo -e "\n5. Updating user:"
  curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
    -H "Content-Type: application/json" \
    -d '{"name": "Juan Pérez Actualizado"}' | jq .

  # Get all users
  echo -e "\n6. Getting all users:"
  curl -s "$BASE_URL/api/users" | jq .
else
  echo "❌ Error creating user"
fi

echo -e "\n✅ Pruebas completadas!"