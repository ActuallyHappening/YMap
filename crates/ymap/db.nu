#!/usr/bin/env nu

# this alias may help for typing a lot
# alias db = nu db.nu

print "This is the db controller script"

let is_desktop = (ls ~/Desktop | length) > 5

# This provides a bunch of environment variables
# including SURREAL_PASS which is the root production password for the db
source ./env.nu

def should_be_server [] {
	if $is_desktop {
		print "You may have executed this from your main computer by accident";
		return
	}
}

# Warns if on digitalocean server or desktop
def should_be_main_computer [] {
	if not $is_desktop {
		print "You may have executed this from the server by accident";
		return
	}
}

def now [] {
	date now | date to-record | get month day hour minute second | $"($in.1) of ($in.0)-($in.2):($in.3):($in.4)"
}

def main [] {
	print "See subcommands"
}

def sshserver [cmd: string] {
	# Add -N at your own ristk
	# It will disable the random extra logs you get, but also doesn't work?
	ssh -f -T digitalocean1 $cmd
}

# requires password to sync annoyingly
def "main sync" [] {
	should_be_main_computer
	# ssh desktop "cd ~/Desktop/YMap/crates/ymap; git pull"
	# scp ./env.nu desktop:~/Desktop/YMap/crates/ymap/env.nu

	# git stash in case uncommitted changes were uploaded in env.nu, db.nu, or db.surql
	ssh digitalocean1 "cd /root/home/YMap/crates/ymap; git stash; git pull"
	scp ~/Desktop/YMap/crates/ymap/env.nu digitalocean1:/root/home/YMap/crates/ymap/env.nu
	scp ~/Desktop/YMap/crates/ymap/db.nu digitalocean1:/root/home/YMap/crates/ymap/db.nu
	scp ~/Desktop/YMap/crates/ymap/db.surql digitalocean1:/root/home/YMap/crates/ymap/db.surql
}


# Runs the actual db
def "main start" [] {
	should_be_server

	print $"Starting surreal db server (now)"

	# by default from env.nu, --bind s to 0.0.0.0:42069
	# let log_path = $"logs/(now):surreal.log";
	/usr/local/bin/surreal start file:surreal.db
	# /usr/local/bin/surreal start file:surreal.db o+e>| $log_path
}

def "main server" [] {
	print "See subcommands [start]"
}

# just starts server, see `db server restart` for proper initialization
def "main server start" [] {
	should_be_main_computer

	print $"Starting server (now)"
	# MARK: CHANGE ME if it is not working as expected
	sshserver "/root/.cargo/bin/nu /root/home/YMap/crates/ymap/db.nu start" o+e> nu.log
}

# imports the db.surql file which defines schemas
def "main server import" [] {
	should_be_main_computer

	print "Importing db.surql"
	# connect to server through env vars
	# only supports http/s not ws
	# see https://github.com/surrealdb/surrealdb/issues/3548
	surreal import ~/Desktop/YMap/crates/ymap/db.surql --endpoint $"http://($env._SURREAL_CONN)"
}

def "main server clean" [] {
	should_be_main_computer

	print "Cleaning files on server"
	sshserver "ps | find surreal | get pid | each {|pid| kill $pid }; rm -rf surreal.db;"
}

def "main server reset" [] {
	should_be_main_computer
	main sync

	main server clean
	main server start

	print "Sleeping for to wait for server"
	sleep 1sec
	print "Finished sleeping"

	main server import

	print "Restarted server"
}

def "main connect" [] {
	surreal sql --pretty --endpoint $env._SURREAL_CONNECTION
}

def "main forwarding" [] {
	print "See db forwarding [start|check]"
}

# def "main forwarding start" [] {
# 	should_be_desktop

# 	print "Starting ssh client in the background, see `ps | find ssh`";

# 	ssh -f -N -T -R 0.0.0.0:8000:localhost:42069 digitalocean-forwarding

# 	print "Now the local port 42069 is open to requests sent to the server on port 8000";
# }

def "main forwarding check" [] {
	ssh -O check digitalocean-forwarding
}

def "main forwarding exit" [] {
	ssh -O exit digitalocean-forwarding
}