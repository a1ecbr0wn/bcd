---
layout: docs
nav_order: 2
---

# How to use

## Create a bookmark

``` sh
# create a bookmark to the logs directory
$ cd /var/log
$ bcd -s logs

# move to home
$ cd ~

# change to the bookmarked directory
$ bcd logs
$ pwd
```

## List bookmarks

``` sh
# list the bookmarks
$ bcd -l
```

## Remove bookmarks

``` sh
# remove a bookmarks
$ bcd -r logs
```

## Help

``` sh
# show the usage/help message
$ bcd -h
```
