<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

[![Crates.io](https://img.shields.io/crates/l/bookmark-cd)](https://github.com/a1ecbr0wn/bcd/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/bookmark-cd)](https://crates.io/crates/bookmark-cd) [![Build Status](https://github.com/a1ecbr0wn/bcd/workflows/CI%20Build/badge.svg)](https://github.com/a1ecbr0wn/bcd/actions/workflows/build.yml) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/bcd/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/bcd)

`bcd` is a way to `cd` to directories that have been bookmarked.

## Installation

### 1: Install bookmark-cd

Currently the way to install `bcd` is via the rust tool cargo:

``` bash
cargo install bookmark-cd
```

### 2: Setup your environment

At the moment, only the `bash` shell is supported, feel free to [contribute](#contribute) if you are reading this
and you use a different shell.

Run the following command to set up your bash environment.

``` bash
bookmark-cd -i
```

What this task is doing is:

- Touching a file to hold your bookmarks in your home folder `.bcd`
- Adding the following command to your `~/.bashrc` file to add the `bcd` shell function when you start your shell:
`eval "$(bookmark-cd init)"`

### Restart your shell

As the shell function creation only gets run when you start a new instance of your shell, you need to start a new
shell before the `bcd` command works.

### Create some bookmarks

``` bash
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
