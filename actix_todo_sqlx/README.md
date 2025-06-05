# Actix TODO API

A simple TODO API built with **Rust** (Actix Web) and **PostgreSQL**.

## Setup

### 1. Start the App

Make sure Docker is installed. Then run:

```bash
docker-compose up -d && cargo run
```
This will start PostgreSQL using Docker and run the Rust server.

Server will be available at: http://127.0.0.1:8080

API documentation: http://127.0.0.1:8080/swagger-ui/

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

