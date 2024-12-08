#!/bin/bash

# Run the CLI with the provided arguments
/app/server "$@"

# Print the arguments
echo "Arguments passed to the CLI: $@"