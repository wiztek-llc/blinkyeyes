#!/usr/bin/env bash
set -e

for i in $(seq 1 20); do
  echo "=== Run $i/20 ==="
  claude --dangerously-skip-permissions -p "@onboarding_auto_build/WORKER_INSTRUCTIONS.md" --output-format stream-json --verbose
  echo ""
done
