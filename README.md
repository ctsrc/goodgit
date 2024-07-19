# `gg` – good git

Why git good when you can good git?

`gg` is a program that clones repos from GitHub and simultaneously retrieves and stores data about the repo,
the owner, and the other repos that the owner has, and more, from the GitHub API.

## Developing on macOS

Run jaeger Docker container with native OTLP ingestion

```zsh
docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
```

Run the gg app

```zsh
cargo run
```

View traces at http://localhost:16686/

