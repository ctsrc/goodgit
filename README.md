# `gg` – good git

[![Crates.io](https://img.shields.io/crates/v/goodgit?style=flat-square)](https://crates.io/crates/goodgit)
[![Crates.io](https://img.shields.io/crates/d/goodgit?style=flat-square)](https://crates.io/crates/goodgit)
[![License](https://img.shields.io/badge/license-ISC-blue?style=flat-square)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/ctsrc/goodgit?style=social)](https://github.com/ctsrc/goodgit#start-of-content)

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
cargo run -- https://github.com/ctsrc/repotools
```

View traces at http://localhost:16686/

