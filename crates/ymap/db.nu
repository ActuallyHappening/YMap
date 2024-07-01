#!/usr/bin/env nu

# This provides a bunch of environment variables
# including SURREAL_PASS which is the root production password for the db
source ./env.nu

# this alias may help for typing a lot
# alias db = nu db.nu
# alias dbf = db forwarding

print "This is the db controller script"


def should_be_server [] {
	if (ls ~/Desktop | length) > 5 {
		print "You may have executed this from your main computer by accident";
		# return
	}
}

# Warns if on digitalocean server or desktop
def should_be_main_computer [] {
	if (ls ~/Desktop | length) < 5 {
		print "You may have executed this from the server by accident";
		# return
	}
}

# no commands should be run on the actual digitalocean server, its just for ssh reverse tunneling

def main [] {
	print "See subcommands"
}

# requires password to sync annoyingly
def "main sync" [] {
	should_be_main_computer
	# ssh desktop "cd ~/Desktop/YMap/crates/ymap; git pull"
	# scp ./env.nu desktop:~/Desktop/YMap/crates/ymap/env.nu

	ssh digitalocean1 "cd /root/home/YMap/crates/ymap; git pull"
}

# Runs the db on server of main computer
def "main start" [] {
	# by default from env.nu, --bind s to 0.0.0.0:42069
	surreal start file:surreal.db
}

def "main server" [] {
	print "See subcommands [start]"
}

def "main server start" [] {
	should_be_main_computer

	ssh digitalocean1 "cd /root/home/YMap/crates/ymap; /root/.cargo/bin/nu db.nu start"
}

def "main server reset" [] {
	should_be_main_computer

	ssh digitalocean1 "cd /root/home/YMap/crates/ymap; rm -rf "surreal.db"; /root/.cargo/bin/nu db.nu start"
}

def "main connect" [] {
	surreal sql --pretty --endpoint ws://actually-happening.foundation:42069
}

# def "main forwarding" [] {
# 	print "See db forwarding [start|check]"
# }

# def "main forwarding start" [] {
# 	should_be_desktop

# 	print "Starting ssh client in the background, see `ps | find ssh`";

# 	ssh -f -N -T -R 0.0.0.0:8000:localhost:42069 digitalocean-forwarding

# 	print "Now the local port 42069 is open to requests sent to the server on port 8000";
# }

# def "main forwarding check" [] {
# 	ssh -O check digitalocean-forwarding
# }

# def "main forwarding exit" [] {
# 	ssh -O exit digitalocean-forwarding
# }