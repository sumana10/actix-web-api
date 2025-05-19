#!/bin/bash

# Send 10 concurrent requests
for i in {1..10}; do
    curl http://127.0.0.1:3000/thread-info &
done

# Wait for all requests to complete
wait
echo "All requests completed"

