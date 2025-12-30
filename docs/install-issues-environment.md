---
layout: docs
nav_order: 5
parent: Installation
---

# Environment Setup

After your installation, you need to restart your shell, your installation should have automatically set up the pointer to `bcd` in your shell startup script.  If the command `bcd` is not working after your installation or something else has happened to change your shell scripts, running `bookmark-cd` and then restarting your shell should set this correctly.

At the moment, only the `bash`, `zsh`, `ksh` and `fish` shells are supported, feel free to [contribute](contribute.md) if you are reading this and you need support for a different shell.

## bash or zsh

The following commmand should be set in your `~/.bashrc` or `~/.zshrc`:

``` sh
# bookmark-cd init block
eval "$(bookmark-cd init)"
eval "$(bookmark-cd completions)"
```

## ksh

The following commmand should be set in your `~/.kshrc`:

``` sh
# bookmark-cd init block
bookmark-cd init > ~/.bcd_ksh
. ~/.bcd_ksh
```

## fish

The following commmand should be set in your `.config/fish/config.fish`:

``` sh
# bookmark-cd init block
bookmark-cd init | source
bookmark-cd completions | source
```

## PowerShell

The following should be set in your '$PROFILE':

``` sh
# bookmark-cd init block
bookmark-cd init | Out-String | Invoke-Expression
bookmark-cd completions | Out-String | Invoke-Expression
```
