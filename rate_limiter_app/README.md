# Actix Web Mutable State Demo Using Mutex

A simple demo showcasing how to manage shared mutable state in Actix Web using `Mutex`.

## Features

- Stores per-IP data using a shared `Mutex<HashMap>`  
- Demonstrates how to build basic middleware-like logic for rate control  
- Limits each IP to 5 requests per minute on a protected endpoint  
- Includes protected and unprotected GET endpoints  

## Running

```bash
cargo run

| Endpoint     | Method | Description                                   |
| ------------ | ------ | --------------------------------------------- |
| `/`          | GET    | Returns "Hello world!" (unprotected)          |
| `/protected` | GET    | Rate-limited endpoint (5 requests/min per IP) |

# Unprotected endpoint
curl http://127.0.0.1:8080/

# Protected endpoint (within limit)
curl http://127.0.0.1:8080/protected

# Exceed rate limit (send more than 5 requests in a minute)
for i in {1..10}; do curl http://127.0.0.1:8080/protected; echo; done

#How It Works

The server uses a simple in-memory rate limiter that:

Extracts the IP address from the request

Stores timestamps of previous requests in a HashMap<IpAddr, Vec<Instant>>

Retains only timestamps from the past 60 seconds

Rejects the request with HTTP 429 if the IP has made 5+ requests in that window