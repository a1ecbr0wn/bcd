# Have the `bcd` command inherit completions from `bookmark-cd`
#
# This needs to be in a seperate file in order to be automatically loaded

complete -c bcd --wraps bookmark-cd
