function bcd
    set result (bookmark-cd $argv)
    switch "$result"
    case "cd*"
        cd (string sub -s 4 $result)
    case '*'
        for s in $result
            echo $s
        end
    end
end
