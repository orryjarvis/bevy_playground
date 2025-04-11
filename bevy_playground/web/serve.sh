#!/bin/bash
set -e

# Navigate to the web directory
cd "$(dirname "$0")"

# Check if Python is installed
if command -v python3 &> /dev/null; then
    python3 -m http.server --bind 0.0.0.0
elif command -v python &> /dev/null; then
    python -m http.server --bind 0.0.0.0
else
    echo "Error: Python is not installed. Please install Python to run the server."
    exit 1
fi
