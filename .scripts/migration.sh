#!/bin/bash

# Define the log file location
LOG_FILE=/app/migration.log

# Run the migration command and log output
diesel migration run --database-url postgresql://postgres:mysecretpassword@db:5432/example > $LOG_FILE 2>&1

# Print the log to stdout
cat $LOG_FILE
