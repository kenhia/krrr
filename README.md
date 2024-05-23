# krrr - Ken's Rust REST Remote

A simple REST server to do simple things remotely.

> Warning: This could be an open door to hacking the machine, so the endpoints
created should do safe simple things and validate the inputs.

Currently just allows for launching remote VS Code. May add more things if I
find the need.

## Remote VS Code
I'm often ssh'd into a remote machine and want to open a folder in VS code from
my host machine. This is a bit of a pain as I have to note or copy the directory
I'm already in on the remote machine, switch to a local terminal and then run my
helper script to start the remote back to the machine I'm ssh'd into.

Decided it would be nicer to have a way to run a command on the remote
machine that would talk to the host machine and ask it to do the needful.

So, this solves the *Host* side of the problem. From the *Remote* machine
I can use curl or write a script to collect the information from my current
directory and send it to the host machine.

### Configuration

Requires either a .env with the below or environment variables set with the
value(s). Only `KRRR_CODE_PATH` is required.

```bash
KRRR_CODE_PATH="C:\\Users\\user\\AppData\\Local\\Programs\\Microsoft VS Code\\bin\\code.cmd"
# Optional, host IP will default to local_ip_address::local_ip()
KRRR_HOST_IP="192.168.1.200"
# Optional, port will default to 42271
KRRR_HOST_PORT="42271"
```

There is an example fish function that calls this server in [rrcode.fish](/rrcode.fish)


