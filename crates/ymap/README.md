## Setup
`ln -s ~/.env/ymap/env.nu env.nu`

### SSH
`-f` run in background
`-N` don't execute any commands
`-T` disables interactive shells

`ssh -R 0.0.0.0:8000:localhost:42069 digitalocean1`
Binds the server's port 8000 to the internal local port 42069

See `nu db.nu port-forward`

See https://stackoverflow.com/questions/1821968/how-do-i-kill-a-backgrounded-detached-ssh-session/26470428#26470428 for advanced management