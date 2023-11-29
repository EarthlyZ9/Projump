export PROJUMP="$HOME/projump"

function projump {
    rust_executable="${PROJUMP}/projump"
    if [ "$#" -eq 1 ]; then
        data_file="${PROJUMP}/.aliases"
        alias_name="$1"
        directory=$(grep "^$alias_name " "$data_file" | awk '{print $2}')
        if [ -n "$directory" ]; then
            echo "$directory"
            cd "$directory" || echo "No such directory."
        else
            echo "No such alias found."
        fi
    else
        "$rust_executable" "$@"
fi
}