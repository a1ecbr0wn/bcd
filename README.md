<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

[![Crates.io](https://img.shields.io/crates/l/bookmark-cd)](https://github.com/a1ecbr0wn/bcd/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/bookmark-cd)](https://crates.io/crates/bookmark-cd) [![Build Status](https://github.com/a1ecbr0wn/bcd/workflows/CI%20Build/badge.svg)](https://github.com/a1ecbr0wn/bcd/actions/workflows/build.yml) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/bcd/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/bcd) [![snapcraft.io](https://snapcraft.io/bookmark-cd/badge.svg)](https://snapcraft.io/bookmark-cd)

`bcd` is a way to `cd` to directories that have been bookmarked.

## Installation

### Homebrew

`bcd` is not currently in the [Homebrew](https://brew.sh/) core repository yet, however you can easily add a tap for `bcd` first:

``` sh
brew tap a1ecbr0wn/bcd
```

then run the install:

``` sh
brew install bcd
```

and then restart your shell.

### Cargo

A simple way for fellow rust developers to install `bcd` is via the rust tool `cargo`:

``` bash
cargo install bookmark-cd
```

### Snapcraft

Snaps are app packages for desktop, cloud and IoT that are easy to install, secure, cross‐platform and dependency‐free. Snaps are discoverable and installable from the Snap Store, the app store for Linux with an audience of millions.

To install `bcd` from snap, run the following:

``` sh
snap install bookmark-cd
```

Snapcraft does not allow write access to the shell startup scripts which are required to set up `bcd` so the following commmand should be set up manually in your `~/.bashrc` if you are using bash or `~/.zshrc` if you are using zsh as your shell:

``` sh
# bookmark-cd init block
eval "$(bookmark-cd init)"   
```

The following should be run so that the snap container allows `bcd` to check that the command has been set up in your shell init file:

``` sh
# For bash
sudo snap connect bookmark-cd:dot-bashrc
# For zsh
sudo snap connect bookmark-cd:dot-zshrc
```

then restart your shell.

### Other package managers

... for any other package managers please raise an [issue](https://github.com/a1ecbr0wn/bcd/issues)

### Installation issues - Setup your environment

After your installation, you need to restart your shell, your installation should have automatically set up the pointer to `bcd` in your shell startup script.  If the command `bcd` is not working after your installation or something else has happened to change your shell scripts, running `bookmark-cd` and then restarting your shell should set this correctly.

At the moment, only the `bash`, `zsh`, `ksh` and `fish` shells are supported, feel free to [contribute](#contribute) if you are reading this and you need support for a different shell.

#### bash or zsh

The following commmand should be set in your `~/.bashrc` or `~/.zshrc`:

``` sh
# bookmark-cd init block
eval "$(bookmark-cd init)"   
```

#### ksh

The following commmand should be set in your `~/.kshrc`:

``` sh
# bookmark-cd init block
bookmark-cd init > ~/.bcd_ksh && . ~/.bcd_ksh
```

#### fish

The following commmand should be set in your `.config/fish/config.fish`:

``` sh
# bookmark-cd init block
bookmark-cd init | source
```

## How to use: create a bookmarks

``` sh
# create a bookmark to the logs directory
cd /var/log
bcd -s logs

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

[![Get it from the Snap Store](https://snapcraft.io/static/images/badges/en/snap-store-black.svg)](https://snapcraft.io/bookmark-cd)
