#! /usr/bin/env nu

# This provides a bunch of environment variables
# including SURREAL_PASS which is the root production password for thd db
source ./env.nu

def main [command: string] {
	if $command == "start" {
		surreal start file:surreal.db
	} else if $command == "connect" {
		surreal sql --pretty --endpoint ws://actually-happening.foundation:8000 
	} else if $command == "port-forward" {
		if (ls ~/Desktop | length) < 5 {
			echo "You may have executed this from your main computer by accident"
			return
		}
		
		ssh -R -f -N -T 0.0.0.0:8000:localhost:42069 digitalocean1
	}
}