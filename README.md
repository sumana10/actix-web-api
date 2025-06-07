# 🦀 Actix Web Demos

A collection of minimal Actix Web projects showcasing various backend patterns in **Rust**.

---

## 📦 Projects Overview

### ✅ [`actix_todo_sqlx`](https://github.com/sumana10/actix-web-api/tree/main/actix_todo_sqlx)
- Basic TODO app with PostgreSQL and `sqlx`
- RESTful routes for task management

### 📘 [`actix_swagger_demo`](https://github.com/sumana10/actix-web-api/tree/main/actix_swagger_demo)
- UUID-based user creation
- Live Swagger documentation using `utoipa`

### 🔒 [`actix_validate_api`](https://github.com/sumana10/actix-web-api/tree/main/actix_validate_api)
- Accepts JSON and form inputs
- Input validation using `validator`
- Custom error handling

### 🧵 [`actix_web_demo`](https://github.com/sumana10/actix-web-api/tree/main/actix_web_demo)
- Multi-threading demo
- Logs current thread ID on each request

### 📝 [`actix-logger-server`](https://github.com/sumana10/actix-web-api/tree/main/actix-logger-server)
- Middleware-based request logging
- Basic CRUD functionality

### 🔐 [`rate_limiter_app`](https://github.com/sumana10/actix-web-api/tree/main/rate_limiter_app)
- In-memory rate limiter using `Mutex<HashMap<IP, Count>>`
- Shared state across threads

### 🚀 [`my_actix_app`](https://github.com/sumana10/actix-web-api/tree/main/my_actix_app)
- Sandbox for trying out new Actix features
- Non-opinionated, exploratory setup

### 📋 [`todo-actix`](https://github.com/sumana10/actix-web-api/tree/main/todo-actix)
- TODO app using in-memory storage
- Shared state handled with `Mutex<Vec<Todo>>`
- No external database, ideal for quick testing and prototyping


### 🛡️ [`actix_jwt_api`](https://github.com/sumana10/actix-web-api/tree/main/actix_jwt_api)
- Secure Notes API with JWT authentication
- Built using Actix Web, PostgreSQL, and sqlx
- Endpoints for user auth and CRUD operations on notes
- Docker + cargo run based local setup
- Includes a bash test script for API testing