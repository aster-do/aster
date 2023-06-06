#!/bin/bash

COLLECTOR_API_PORT=3035

# Array of possible states
states=("cpu" "ram" "disk" "network")

while true; do
  # Generate a random index to select a state from the array
  index=$(( RANDOM % ${#states[@]} ))

  # Select a random state
  state=${states[$index]}

  # Generate a random value between 0 and 2500
  value=$(( RANDOM % 2501 ))

  # Generate the current timestamp in the specified format
  timestamp=$(date -u +"%Y-%m-%dT%H:%M:%S%z")

  # Generate a random correlation_id between 0 and 10000
  corelation_id=$(( RANDOM % 10001 ))

  # Print the variables
  echo "Send a metric with Corelation_id: $corelation_id, Timestamp: $timestamp, State: $state and Value: $value"

  # Send the metric to the
  curl -X POST "localhost:3035/metrics" -H "Content-Type: application/json" \
    --data-raw "{\"corelation_id\": \"$corelation_id\",\"name\":\"$state\",\"timestamp\": \"$timestamp\", \"value\":$value}"

  # Delay for 100 milliseconds
  sleep .1

done