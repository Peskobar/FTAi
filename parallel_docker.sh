#!/usr/bin/env bash
# parallel_docker.sh: Split tasks and launch multiple Docker workers in parallel.
set -euo pipefail

usage() {
  cat <<EOF
Usage: $0 <NUM_CONTAINERS> <TASK_FILE> <IMAGE>
Example: $0 4 tasks/tasks.txt smartfix/worker:latest
EOF
  exit 1
}

# Args
[[ $# -lt 3 ]] && usage
NUM_CONTAINERS="$1"
TASK_FILE="$2"
IMAGE="$3"
CHUNK_DIR="tasks/chunks"

[[ ! "$NUM_CONTAINERS" =~ ^[0-9]+$ ]] && echo "Error: NUM_CONTAINERS must be integer." >&2 && exit 1
[[ ! -f "$TASK_FILE" ]] && echo "Error: Task file not found." >&2 && exit 1

# Prepare chunks
rm -rf "$CHUNK_DIR" && mkdir -p "$CHUNK_DIR"
split -d -n l/"$NUM_CONTAINERS" "$TASK_FILE" "$CHUNK_DIR/chunk_"

# Launch containers
failures=0
for idx in $(seq -f "%02g" 0 $((NUM_CONTAINERS-1))); do
  CHUNK="$CHUNK_DIR/chunk_$idx"
  [[ ! -s "$CHUNK" ]] && echo "Warning: $CHUNK empty, skipping." >&2 && continue
  name="worker-$idx"
  docker run -d --name "$name" -v "$(pwd)/$CHUNK:/app/tasks.txt:ro" "$IMAGE"
done

echo "Waiting for containers to finish..."
for c in $(docker ps -a --filter "name=worker-" --format "{{.Names}}"); do
  docker wait "$c" || ((failures++))
done

echo "Completed. Failures: $failures"
exit "$failures"
