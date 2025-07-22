# agent_docker.md

## Purpose
Automate parallel execution of tasks in Docker containers using GPT‑Codex.

## Prerequisites
- Docker ≥ 20.10 with Compose plugin.
- `tasks/tasks.txt` with one task per line.
- Docker image `smartfix/worker:latest` built from `backend/Dockerfile`.

## Provided Files
- `docker-compose.yml`
- `parallel_docker.sh`
- `tasks/tasks.txt` *(user-provided)*

## Quick Start
```bash
# Build worker image if not available
docker compose build worker
# Split tasks into 4 chunks and launch
./parallel_docker.sh 4 tasks/tasks.txt smartfix/worker:latest
```

## Script: parallel_docker.sh
1. Validates args: NUM_CONTAINERS, TASK_FILE, IMAGE.
2. Splits TASK_FILE into N balanced chunks: `split -n l/N`.
3. Launches N containers with names `worker-00…worker-<N-1>`:
   ```bash
   docker run -d --name worker-<idx> \
     -v "$(pwd)/tasks/chunks/chunk_<idx>:/app/tasks.txt:ro" \
     --restart on-failure:3 \
     "${IMAGE}"
   ```
4. Waits for all containers to finish and reports exit codes.

## Compose Scaling
```bash
# scale to 8 replicas
docker compose up -d --scale worker=8
# view logs
docker compose logs -f
# stop and remove
docker compose down
```

## Docker Swarm & Kubernetes
- **Swarm**: `docker stack deploy -c docker-compose.yml smartfix --with-registry-auth --replicas 5`.
- **K8s**: Use Helm chart in `deploy/helm/smartfix`, then:
  ```bash
  helm install smartfix deploy/helm/smartfix
  kubectl scale deployment smartfix-backend --replicas=5
  ```

## Cleanup
```bash
docker rm -f $(docker ps -a -q --filter "name=worker-")
rm -rf tasks/chunks
```

## Debugging & Monitoring
```bash
# inspect logs
docker logs worker-00
# exec into a container
docker exec -it worker-01 sh
```
