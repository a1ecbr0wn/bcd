---
layout: docs
title: "Snapcraft Installation | bookmark-cd | bcd"
nav_order: 2
parent: Installation
---
# Snapcraft Installation

Snaps are app packages for desktop, cloud and IoT that are easy to install, secure, cross‐platform and dependency‐free. Snaps are discoverable and installable from the Snap Store, the app store for Linux with an audience of millions.

To install `bcd` from snap, run the following:

``` sh
snap install bookmark-cd
```

Snapcraft does not allow write access to the shell startup scripts which are required to set up `bcd` so the following commmand should be set up manually in your shell init script, e.g, `~/.bashrc` if you are using bash, `~/.zshrc` if you are using zsh as your shell, etc.:

``` sh
# bookmark-cd init block
eval "$(bookmark-cd init)"
eval "$(bookmark-cd completions)"
```

The following should be run so that the snap container allows `bcd` to check that the command has been set up in your shell init file:

``` sh
# For bash
sudo snap connect bookmark-cd:dot-bashrc
# For Fish
sudo snap connect bookmark-cd:dot-config-fish
# For ksh
sudo snap connect bookmark-cd:dot-kshrc
# For zsh
sudo snap connect bookmark-cd:dot-zshrc
```

then restart your shell.

Powershell is not supported as the location of `$PROFILE` can vary but snapcraft requires it to be logged as a specific location.
