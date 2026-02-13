#!/usr/bin/env bash
set -e

for i in $(seq 1 25); do
  echo "=== Run $i/25 ==="
  claude --dangerously-skip-permissions -p "@saas_ready_auto_build/WORKER_INSTRUCTIONS.md" --output-format stream-json --verbose
  echo ""
done
