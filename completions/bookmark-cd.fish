# Hand-written completions for the fish shell
#
# NOTE: Prefer builtin functions to external commands,
# they are usally an order maginitude faster

# Command substitution to list bookmarks
set --local docomplete_bookmarks '$(string join \n $(string replace , \t < ~/.bcd)[2..])'

# Disable file completions for entire command
complete -c bookmark-cd -f

complete -c bookmark-cd -d "Bookmarked directory to change to" -a "$docomplete_bookmarks"

# TODO: Should we hint for bookmarks when specifying incompatible flag like --store
complete -c bookmark-cd -s s -l store -d "Store the current directory as a bookmark" -r
complete -c bookmark-cd -s r -l remote -d "Remove a specified bookmark" -ra "$docomplete_bookmarks"
complete -c bookmark-cd -s l -l list -d 'List the stored bookmarks'
complete -c bookmark-cd -s V -l version -d 'Print version information'
complete -c bookmark-cd -s h -l help -d 'Print help'

