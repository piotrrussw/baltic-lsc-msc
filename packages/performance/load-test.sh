#!/bin/bash

# Test React SSR App
echo "Testing React App - 10 users 1 minute"
siege -c 10 -t 1M http://localhost:3000/

sleep 30

echo "Testing React App - 100 users 1 minute"
siege -c 100 -t 1M http://localhost:3000/

sleep 30

echo "Testing React App - 1000 users 1 minute"
siege -c 1000 -t 1M http://localhost:3000/

sleep 30

# TODO yew ssr build
# Test Yew SSR App
# echo "Testing Yew App - 10 users 1 minute"
# siege -c 10 -t 1M http://localhost:your-yew-app-endpoint

# sleep 30

# echo "Testing Yew App - 100 users 1 minute"
# siege -c 100 -t 1M http://localhost:your-react-app-endpoint

# sleep 30

# echo "Testing Yew App - 1000 users 1 minute"
# siege -c 1000 -t 1M http://localhost:your-react-app-endpoint

# sleep 30