# `gg` – good git

[![Crates.io](https://img.shields.io/crates/v/goodgit?style=flat-square)](https://crates.io/crates/goodgit)
[![Crates.io](https://img.shields.io/crates/d/goodgit?style=flat-square)](https://crates.io/crates/goodgit)
[![License](https://img.shields.io/badge/license-ISC-blue?style=flat-square)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/ctsrc/goodgit?style=social)](https://github.com/ctsrc/goodgit#start-of-content)

Why git good when you can good git?

`gg` is a program that clones repos from GitHub and simultaneously retrieves and stores data about the repo,
the owner, and the other repos that the owner has, and more, from the GitHub API. Oh and it also works with Gitlab.

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

## Ancient history

`gg` is the successor of one small pair of shell functions.

While the functions were still in use, here is what they most recently looked like:

```zsh
func ghu () {
  # TODO: More robust
  user="$( echo "$1" | sed 's#^https://github.com/\([^/]*\)/.*#\1#' )"
  ghudir="$HOME/src/github.com/$user"
  [[ -d "$ghudir" ]] || mkdir -p "$ghudir"
  cd "$ghudir"
}

func gh () {
  ghu "$1"
  #url="$( echo "$1" | sed 's#^https://github.com/\([^/]*\)/\([^/]*\).*#git@github.com:\1/\2.git#' )"
  url="$( echo "$1" | sed 's#^https://github.com/\([^/]*\)/\([^/]*\).*#https://github.com/\1/\2.git#' )"
  echo "$url"
  ts git clone --bare "$url"
}
```

Shell regexes were used, and then a bare clone was enqueued using task spooler.
<https://www.freshports.org/sysutils/ts/>

It was a humble beginning. It did the most important thing which was to schedule git clone.
But it did not retrieve additional info about the repo or the owner from API.
Nor did it support Gitlab URLs.

It was time for something better. It was time for `gg` – good git.

