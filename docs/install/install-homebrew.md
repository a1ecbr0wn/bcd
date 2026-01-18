---
layout: docs
title: "Homebrew Installation | bookmark-cd | bcd"
nav_order: 1
parent: Installation
---

## Homebrew Installation

`bcd` is not currently in the [Homebrew](https://brew.sh/) core repository yet,
however you can easily add a tap for `bcd` first:

```sh
brew tap a1ecbr0wn/bcd
```

then run the install:

```sh
brew install bcd
```

and then restart your shell.

If you are using a shell other than `bash`, you will need to run the following
command to ensure correct [environment setup](install-issues-environment.md):

```sh
bookmark-cd -i
```
