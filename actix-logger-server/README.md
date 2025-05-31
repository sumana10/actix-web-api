# User API with Actix-web

A simple RESTful User API built in Rust using Actix-web. Supports basic CRUD operations with mock data and custom request logging.

## Features

- Create, read, and delete users
- JSON request/response handling with Serde
- Request logging middleware showing method, path, and response time

## Endpoints

| Method | Path         | Description          |
|--------|--------------|----------------------|
| GET    | `/`          | Hello world message   |
| GET    | `/health`    | Health check         |
| GET    | `/users`     | List all users       |
| GET    | `/users/{id}`| Get user by ID       |
| POST   | `/users`     | Create a new user    |
| DELETE | `/users/{id}`| Delete user by ID    |

## Data Models

```rust
struct User {
  id: u32,
  name: String,
  email: String,
}

struct CreateUser {
  name: String,
  email: String,
}
```

## Run the Project
```
cargo run
```
Server runs on 127.0.0.1:8080.

