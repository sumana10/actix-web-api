# Actix TODO API

A simple TODO API built with **Rust** (Actix Web) and **PostgreSQL**.

## Setup

### 1. Start PostgreSQL with Docker

```bash
docker run --name postgres-todo \
  -e POSTGRES_USER=todouser \
  -e POSTGRES_PASSWORD=todopass \
  -e POSTGRES_DB=todoapp \
  -p 5433:5432 \
  -d postgres:16
```
### 2. Run the App

```
cargo run
```
Server will run at: http://127.0.0.1:8080

## API Endpoints

- GET /health – Health check

- GET /api/v1/todos – Get all todos

- POST /api/v1/todos – Create a todo

- GET /api/v1/todos/{id} – Get a todo by ID

- PUT /api/v1/todos/{id} – Update a todo

- DELETE /api/v1/todos/{id} – Delete a todo

### Test

```
chmod +x test_api.sh
./test_api.sh
```

