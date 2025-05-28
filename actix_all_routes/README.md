# Actix: All Types of Routes

A simple REST API built with Rust and Actix-web.  
Supports user CRUD, query parameters, nested routes, and scoped routes.

## Features

- Basic routes (home, health check)  
- Create, read, update, delete users  
- Query params for pagination & search  
- Nested routes for user posts  
- Scoped routes for API versioning & admin  
- JSON input/output with Serde  

## Endpoints

| Method | Path                          | Description                  |
|--------|-------------------------------|------------------------------|
| GET    | `/`                           | Welcome message              |
| GET    | `/health`                     | Health check                |
| GET    | `/users`                      | List users                  |
| POST   | `/users`                      | Create user                 |
| GET    | `/users/{id}`                 | Get user by ID              |
| PUT    | `/users/{id}`                 | Update user by ID           |
| DELETE | `/users/{id}`                 | Delete user by ID           |
| GET    | `/users/{id}/posts`           | List posts for a user       |
| GET    | `/users/{user_id}/posts/{post_id}` | Get a specific post for a user |

Scoped routes available under `/api/v1` and `/admin`

## Models

```rust
struct User {
  id: u32,
  name: String,
  email: String,
}

struct Post {
  id: u32,
  title: String,
  user_id: u32,
}

struct UserQuery {
  page: Option<u32>,
  limit: Option<u32>,
  search: Option<String>,
}

struct CreateUser {
  name: String,
  email: String,
}
```
# Run

```
cargo run
```
Server listens on 127.0.0.1:8080

## API Testing with curl

### Basic Routes
```
curl -X GET http://127.0.0.1:8080/
curl -X GET http://127.0.0.1:8080/health
```

### User CRUD
```
curl -X GET http://127.0.0.1:8080/users/1
curl -X POST http://127.0.0.1:8080/users \
     -H "Content-Type: application/json" \
     -d '{"name":"Alice","email":"alice@test.com"}'
curl -X PUT http://127.0.0.1:8080/users/1 \
     -H "Content-Type: application/json" \
     -d '{"name":"Aurora","email":"aurora@test.com"}'
curl -X DELETE http://127.0.0.1:8080/users/1
```
### Query Params on GET /users
```
curl -X GET "http://127.0.0.1:8080/users?page=2&limit=5&search=alice"
```
### Nested Routes (posts of a user)
```
curl -X GET http://127.0.0.1:8080/users/1/posts
curl -X GET http://127.0.0.1:8080/users/1/posts/2
```
### Scoped routes under /api/v1
```
curl -X GET http://127.0.0.1:8080/api/v1/users
curl -X GET http://127.0.0.1:8080/api/v1/users/1
```
### Scoped routes under /admin
```
curl -X GET http://127.0.0.1:8080/admin/users
curl -X GET http://127.0.0.1:8080/admin/health
```
