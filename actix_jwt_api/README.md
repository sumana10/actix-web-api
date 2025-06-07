# Actix Notes API

A secure **Notes API** built with **Rust** (Actix Web), **PostgreSQL**, and **JWT Authentication**.

## Setup

### 1. Start the App

Make sure Docker is installed. Then run:

```bash
docker-compose up -d && cargo run
```

This will start PostgreSQL using Docker and run the Rust server.

Server will be available at: http://127.0.0.1:8080

## API Endpoints

### Authentication
- `POST /auth/register` – Register a new user
- `POST /auth/login` – Login and get JWT token

### Notes (Requires JWT Token)
- `GET /health` – Health check
- `GET /notes` – Get all user's notes
- `POST /notes` – Create a new note
- `GET /notes/{id}` – Get a specific note
- `PUT /notes/{id}` – Update a note
- `DELETE /notes/{id}` – Delete a note

### Authentication Header
```
Authorization: Bearer <your-jwt-token>
```

## Test

```bash
chmod +x test_script.sh
./test_script.sh
```
