if [ "$1" == "-p" ] || [ "$1" == "--profile" ] || [[ "$1" =~ ^@.* ]]
then
    if [[ "$1" =~ ^@.* ]]
    then
        eval "$(orunner -p ${1:1} make-command ${@:2})"
    else
        eval "$(orunner -p $2 make-command ${@:3})"
    fi
else
    eval "$(orunner make-command $@)"
fi