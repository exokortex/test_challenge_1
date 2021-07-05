#!/bin/bash

# Start Gunicorn processes
echo Starting Gunicorn.

exec gunicorn server:app \
    --bind 0.0.0.0:5000 \
    --workers 4
