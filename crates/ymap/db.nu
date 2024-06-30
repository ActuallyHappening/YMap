#! /usr/bin/env nu

# This provides a bunch of environment variables
# including SURREAL_PASS which is the root production password for thd db
source ./env.nu

def main [command: string] {
	if $command == "start" {
		surreal start file:surreal.db
	} else if $command == "connect" {
		surreal sql --pretty --endpoint ws://actually-happening.foundation:8000 
	}
}