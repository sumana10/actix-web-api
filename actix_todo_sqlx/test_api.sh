#!/bin/bash

echo "=== Testing Todo API ==="
echo

echo "1. Health Check:"
curl -s http://127.0.0.1:8080/health
echo -e "\n"

echo "2. Create Todo 1:"
curl -s -X POST http://127.0.0.1:8080/api/v1/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust",
    "description": "Complete the Rust tutorial",
    "priority": "high"
  }' | jq '.'
echo

echo "3. Create Todo 2:"
curl -s -X POST http://127.0.0.1:8080/api/v1/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Buy groceries",
    "description": "Milk, bread, eggs",
    "priority": "medium"
  }' | jq '.'
echo

echo "4. List all todos:"
curl -s http://127.0.0.1:8080/api/v1/todos | jq '.'
echo

echo "5. Get specific todo (ID 1):"
curl -s http://127.0.0.1:8080/api/v1/todos/1 | jq '.'
echo

echo "6. Update todo (mark as completed):"
curl -s -X PUT http://127.0.0.1:8080/api/v1/todos/1 \
  -H "Content-Type: application/json" \
  -d '{
    "completed": true,
    "priority": "urgent"
  }' | jq '.'
echo

echo "7. List all todos after update:"
curl -s http://127.0.0.1:8080/api/v1/todos | jq '.'
echo

echo "8. Delete todo (ID 2):"
curl -s -X DELETE http://127.0.0.1:8080/api/v1/todos/2
echo "Status: $?"
echo

echo "9. List all todos after delete:"
curl -s http://127.0.0.1:8080/api/v1/todos | jq '.'
echo

echo "10. Try to get deleted todo (should return 404):"
curl -s http://127.0.0.1:8080/api/v1/todos/2
echo -e "\n"

echo "=== Testing Complete ==="
