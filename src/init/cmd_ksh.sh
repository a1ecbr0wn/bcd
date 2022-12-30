function bcd {
    result=$(bookmark-cd $@ 2>&1)
    if [ "cd" = ${result:0:2} ]; then
        cd ${result:3:${#result}-3}
    else
        echo "$result"
    fi
}