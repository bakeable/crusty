#!/usr/bin/env bash

# Use this script to test if a given TCP host/port are available

TIMEOUT=120
QUIET=0
HOST="$1"
PORT="$2"
shift 2

while [[ $# -gt 0 ]]; do
  case "$1" in
    -q|--quiet)
      QUIET=1
      shift
      ;;
    -t|--timeout)
      TIMEOUT="$2"
      shift 2
      ;;
    *)
      break
      ;;
  esac
done

start_ts=$(date +%s)
end_ts=$((start_ts + TIMEOUT))

while :
do
  if [[ "$(date +%s)" -ge "$end_ts" ]]; then
    echo "Operation timed out" >&2
    exit 1
  fi

  nc -z "$HOST" "$PORT" >/dev/null 2>&1
  result=$?

  if [[ $result -eq 0 ]]; then
    break
  fi

  if (( ( $(date +%s) - start_ts ) % 5 == 0 )); then
    echo "Still waiting for host $HOST and port $PORT..." >&2
  fi

  sleep 1
done

echo "Host $HOST and port $PORT are available"
