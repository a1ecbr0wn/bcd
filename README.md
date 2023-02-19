---
layout: default
toc: true
---
<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

[![Crates.io](https://img.shields.io/crates/l/bookmark-cd)](https://github.com/a1ecbr0wn/bcd/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/bookmark-cd)](https://crates.io/crates/bookmark-cd) [![Build Status](https://github.com/a1ecbr0wn/bcd/workflows/CI%20Build/badge.svg)](https://github.com/a1ecbr0wn/bcd/actions/workflows/build.yml) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/bcd/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/bcd) [![snapcraft.io](https://snapcraft.io/bookmark-cd/badge.svg)](https://snapcraft.io/bookmark-cd)

`bcd` is a way to `cd` to directories that have been bookmarked.

<script id="asciicast-549018" src="https://asciinema.org/a/549018.js" async></script>

## How to use: create a bookmark

``` sh
# create a bookmark to the logs directory
$ cd /var/log
$ bcd -s logs

# move to home
$ cd ~

# list the bookmarks
$ bcd -l

# change to the bookmarked directory
$ bcd logs
$ pwd
```

## Installation

- [Homebrew](docs/install-homebrew.md)
- [Snap](docs/install-snapcraft.md)
- [Cargo](docs/install-cargo.md)
- [Other Package Managers](docs/install-other.md)

[Installation Issues](docs/install-issues.md)

## Contribute

There are many shell that this tool could be used on, I just don't use them.  I would be happy to take a look at any
PRs that add support for other shells.

[![Get it from the Snap Store](https://snapcraft.io/static/images/badges/en/snap-store-black.svg)](https://snapcraft.io/bookmark-cd)
