# Todo API with Actix-web

A simple RESTful Todo API built in Rust using Actix-web. Supports basic CRUD operations with in-memory storage.

## Features

- Create, read, update, and delete todos
- Thread-safe in-memory storage using `Mutex<HashMap>`
- JSON request/response handling with Serde

## Endpoints

| Method | Path          | Description                |
|--------|---------------|----------------------------|
| GET    | `/todos`      | List all todos             |
| POST   | `/todos`      | Create a new todo          |
| GET    | `/todos/{id}` | Get a todo by ID           |
| PUT    | `/todos/{id}` | Update a todo by ID        |
| DELETE | `/todos/{id}` | Delete a todo by ID        |

## Data Models

```rust
struct Todo {
  id: u32,
  title: String,
  completed: bool,
}

struct CreateTodo {
  title: String,
}

struct UpdateTodo {
  title: Option<String>,
  completed: Option<bool>,
}
```

# Running
```
cargo run
```
Server runs on 127.0.0.1:8080.


