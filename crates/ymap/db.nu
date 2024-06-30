#!/usr/bin/env nu

# This provides a bunch of environment variables
# including SURREAL_PASS which is the root production password for thd db
source ./env.nu

print "This is the db controller script"

def main [] {
	print "See subcommands"
}

def "main start" [] {
	surreal start file:surreal.db
}

def "main connect" [] {
	surreal sql --pretty --endpoint ws://actually-happening.foundation:8000
}

def "main forwarding" [] {
	print "See db forwarding [start|check]"
}

def "main forwarding start" [] {
	if (ls ~/Desktop | length) > 5 {
			print "You may have executed this from your main computer by accident";
			# return
		}

		print "Starting ssh client in the background, see `ps | find ssh`";

		ssh -f -N -T -R 0.0.0.0:8000:localhost:42069 digitalocean-forwarding

		print "Now the local port 42069 is open to requests sent to the server on port 8000";
}

def "main forwarding check" [] {
	ssh -O check digitalocean-forwarding
}

def "main forwarding exit" [] {
	ssh -O exit digitalocean-forwarding
}