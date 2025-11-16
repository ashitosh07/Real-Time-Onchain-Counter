#!/bin/bash

# Run the Counter Service
echo "Starting Counter Service..."

# Replace APP_ID with your actual application ID from deployment
APP_ID=${1:-"your-app-id-here"}

if [ "$APP_ID" = "your-app-id-here" ]; then
    echo "Usage: ./run-service.sh <APPLICATION_ID>"
    echo "Please provide the application ID from deployment"
    exit 1
fi

echo "Running service for application: $APP_ID"

linera service --port 8080 --application-id $APP_ID