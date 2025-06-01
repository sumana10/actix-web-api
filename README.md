# 🦀 Actix Web Demos

A collection of minimal Actix-web projects showcasing various backend patterns in Rust.

---

## 📦 Projects Overview

### ✅ Basic REST API
- Users and Posts CRUD
- Route scoping: `/api/v1`, `/admin`
- Query params, nested routes

### 📘 Swagger API
- UUID-based user creation
- Live Swagger documentation using `utoipa`

### 🔒 Validation API
- Accepts JSON and form inputs
- Validations with `validator`
- Custom error handling

### 🧵 Multi-threading Demo
- CPU-core-based threading
- Prints thread ID per request

### 📝 Logging API
- Middleware-based request logging
- Simple user CRUD

### 🔐 Shared State with Mutex
- In-memory rate limiter per IP
- Uses `Mutex<HashMap<IP, Count>>`

