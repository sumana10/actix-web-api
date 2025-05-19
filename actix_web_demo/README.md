# Actix Web Multi-threading Demo

A simple demo showcasing Actix Web's multi-threading capabilities.

## Features

- Spawns one worker thread per CPU core  
- Displays the handling thread ID for each request  
- Includes basic GET and POST endpoints  

## Installation

```bash
cargo run 

| Endpoint       | Method | Description                               |
| -------------- | ------ | ----------------------------------------- |
| `/`            | GET    | Returns "Hello world"                     |
| `/hey`         | GET    | Returns "Hey there"                       |
| `/echo`        | POST   | Echoes back the request body              |
| `/thread-info` | GET    | Shows which worker thread handled request |

Testing

# Basic endpoints
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/hey
curl -X POST --data "Hello from curl!" http://127.0.0.1:3000/echo

# Thread info
curl http://127.0.0.1:3000/thread-info

# Check load distribution across threads
for i in {1..10}; do curl http://127.0.0.1:3000/thread-info; echo; done

# How It Works
The server spawns a separate worker per core:

HttpServer::new(|| {
    App::new()
        // Define routes here
})
.workers(num_cpus::get()) // One worker per CPU core

Each worker runs independently, handling incoming requests concurrently. 