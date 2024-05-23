# Example of calling krrr using fish shell. This will start a remote VS Code
# from the host to the ssh'd client in the current directory.
function rrcode --description 'call krrr via curl'
    set -l host (string split -f 1 ' ' $SSH_CLIENT)
    echo "Calling krrr on $host with USER=$USER and PWD=$PWD"
    curl --request GET "http://$host:42271/rcode?client=karch9&user=$USER&path=$PWD"
end
