# Actix: User API with Validations

A lightweight REST API in Rust using Actix-web.  
Includes JSON & form-based input, route-level validation, and custom error handling.

## Features

- Create & fetch users using JSON or form data  
- Route-level validation with `validator` crate  
- Shared state with `Mutex` and `web::Data`  
- Custom error types and responses  
- Logging middleware with `env_logger`

## Endpoints

| Method | Path            | Description                |
|--------|------------------|---------------------------|
| POST   | `/users/json`    | Create user via JSON      |
| POST   | `/users/form`    | Create user via form data |
| GET    | `/users`         | List all users            |
| GET    | `/users/{id}`    | Get user by ID            |

## Models

```rust
#[derive(Deserialize, Validate)]
struct CreateUser {
  #[validate(length(min = 1))]
  name: String,

  #[validate(email)]
  email: String,
}

struct User {
  id: u32,
  name: String,
  email: String,
}

struct UserResponse {
  user: User,
  message: String,
  source: String,
}
```
## Run the Server

```
cargo run
```
Server runs on: 127.0.0.1:8080

## API Testing with curl

### JSON User Creation
```
curl -X POST http://127.0.0.1:8080/users/json \
  -H "Content-Type: application/json" \
  -d '{"name":"Pixy","email":"pixy@example.com"}'
```
### Form User Creation
```
curl -X POST http://127.0.0.1:8080/users/form \
  -d "name=Pixy&email=pixy@example.com"
```
### Get All Users
```
curl http://127.0.0.1:8080/users
```
### Get User by ID
```
curl http://127.0.0.1:8080/users/1
```