# AGENTS.md

## Overview
Provide GPT‑Codex with comprehensive, step-by-step instructions to:
1. Extract canonical files.
2. Organize a modular repository.
3. Build, test, and lint code.
4. Deploy via Docker Compose, Swarm, or Kubernetes.
5. Split tasks and orchestrate parallel Docker containers.

## 1. Initial File Extraction
```shell
unzip smartfix_dump.zip -d smartfix
```
- Locate and move:
  - `exported-assets (4)/Cargo.toml` → `backend/`
  - `exported-assets (4)/*.rs, config.toml` → `backend/src/`
  - `exported-assets (10)/*.html,*.css,app*.js` → `frontend/`
  - `exported-assets (4)/deployment.yaml` → `deploy/kubernetes/`
  - `exported-assets (4)/*.md` → `docs/`
- Remove all other `exported-assets*` directories and stub files.

## 2. Repository Layout
```
smartfix/
├── backend/
│   ├── Cargo.toml
│   ├── config.toml
│   └── src/*.rs
├── frontend/
│   └── *.html,*.css,app*.js
├── deploy/
│   └── kubernetes/deployment.yaml
└── docs/
    └── ARCHITECTURE.md, EDGE_GUIDE.md, OBSERVABILITY.md, README.md
```

## 3. Build & Test Backend
```bash
cd backend
cargo build --release
cargo test --workspace -- --nocapture
cargo fmt -- --check
```

## 4. Validate Kubernetes Manifests
```bash
kubectl apply -f deploy/kubernetes --dry-run=client
```

## 5. Frontend Local Preview
```bash
cd frontend
python3 -m http.server 8080
# open http://localhost:8080
```

## 6. Docker Compose Deployment
```bash
docker compose up -d --build
docker compose ps
docker compose logs -f
```

## 7. Docker Swarm Deployment
```bash
docker swarm init
docker stack deploy -c docker-compose.yml smartfix --with-registry-auth --replicas 3
```

## 8. Kubernetes Helm Deploy
```bash
helm upgrade --install smartfix deploy/helm/smartfix       --set image.tag=latest --set replicaCount=3
```

## 9. Parallel Task Execution
```bash
# split & run 5 containers
./parallel_docker.sh 5 tasks/tasks.txt smartfix/worker:latest
```

## 10. CI/CD
- **GitHub Actions**: `.github/workflows/ci.yml`
  ```yaml
  name: CI
  on: [push, pull_request]
  jobs:
    backend:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
        - run: cargo test --workspace --release
    frontend:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - name: Install Python
          uses: actions/setup-python@v4
          with:
            python-version: '3.x'
        - run: |
            cd frontend
            pip install --upgrade pip
            python3 -m http.server --check
  ```
- **Linting**: `cargo fmt`, `eslint`.

## 11. Cleanup
```bash
docker rm -f $(docker ps -q --filter "name=worker-")
rm -rf tasks/chunks
```

## 12. Troubleshooting
- Check container logs: `docker logs <name>`
- Exec shell: `docker exec -it <name> sh`
- Inspect Kubernetes events: `kubectl get events`
