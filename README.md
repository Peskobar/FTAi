# TUI-Patcher-Agent

Ten projekt zawiera mikroserwis do inteligentnego łatania aplikacji Android z obsługą telemetrii oraz prosty interfejs webowy. Dokumentacja techniczna znajduje się w katalogu `docs/`.

## Kompilacja backendu

```bash
cd backend
cargo build --release
```

## Testy backendu

```bash
cd backend
cargo test --workspace -- --nocapture
```

## Uruchomienie frontendu

```bash
cd frontend
python3 -m http.server 8080
# otwórz http://localhost:8080 w przeglądarce
```

## Wdrożenie przez Docker Compose

```bash
docker compose up -d --build
docker compose ps
docker compose logs -f
```

## Dokumentacja

- [Architektura](docs/ARCHITECTURE.md)
- [Wdrożenie na Edge](docs/EDGE_GUIDE.md)
- [Obserwowalność](docs/OBSERVABILITY.md)
- [Rozszerzona dokumentacja](docs/README.md)

