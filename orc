if [ "$1" == "-p" ] || [ "$1" == "--profile" ]
then
    eval "$(orunner -p $2 make-command ${@:3})"
else
    eval "$(orunner make-command $@)"
fi