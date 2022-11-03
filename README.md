<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

[![Crates.io](https://img.shields.io/crates/l/bookmark-cd)](https://github.com/a1ecbr0wn/bcd/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/bookmark-cd)](https://crates.io/crates/bookmark-cd) [![Build Status](https://github.com/a1ecbr0wn/bcd/workflows/CI%20Build/badge.svg)](https://github.com/a1ecbr0wn/bcd/actions/workflows/build.yml) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/bcd/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/bcd)

`bcd` is a way to `cd` to directories that have been bookmarked.

## Installation

### Homebrew

`bcd` is not currently in the [Homebrew](https://brew.sh/) core repository yet, however you can easily add a tap for `bcd` first:

``` sh
brew tap a1ecbr0wn/bcd
```

and then run the install:

``` sh
brew install bcd
```

Run this command to set up your shell (bash or zsh):

``` sh
bookmark-cd
```

and then restart your shell

### Cargo

A simple way for fellow rust developers to install `bcd` is via the rust tool `cargo`:

``` bash
cargo install bookmark-cd
```

### Snapcraft

Working on it...  for any other package managers please raise an [issue](https://github.com/a1ecbr0wn/bcd/issues)

### Installation issues - Setup your environment

After your installation, you need to restart your shell, your installation should have automatically set up the pointer to `bcd` in your shell init script.  If the command `bcd` is not working after your installation or something else has happened to change your shell scripts, running `bookmark-cd` and then restarting your shell should set this correctly.

At the moment, only the `bash` and `zsh` shells are supported, feel free to [contribute](#contribute) if you are reading this
and you need support for a different shell.

The following commmand should be set in your `~/.bashrc` or `~/.zshrc`:

``` sh
# bookmark-cd init block
eval "$(bookmark-cd init)"   
```

## How to use: create a bookmarks

``` sh
# create a bookmark to the logs directory
cd /var/log
bcd logs

# move to home
cd ~

# list the bookmarks
bcd -l

# change to the bookmarked directory
bcd logs
pwd
```

## Contribute

There are many shell that this tool could be used on, I just don't use them.  I would be happy to take a look at any
PRs that add support for other shells.
