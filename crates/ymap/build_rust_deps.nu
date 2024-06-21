#!/use/bin/env nu

# based on https://github.com/bevyengine/bevy/blob/main/examples/mobile/build_rust_deps.sh

let is_release = (
	if ($env | get --ignore-errors CONFIGURATION == null) {
		print "No 'CONFIGURATION' environment variable"; 
		return false
	} else if $env.CONFIGURATION == "Debug" {
		print $"'CONFIGURATION' environment variable is ($env.CONFIGURATION) which is  'Debug' so not using release profile"
		return false
	} else {
		print $"'CONFIGURATION' env var is ($env.CONFIGURATION) which is not 'Debug', assuming using release profile"
		return true
	}
)

print "Is release?" $is_release

# for me is /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs
let developer_sdk_dir = $env | get --ignore-errors DEVELOPER_SDK_DIR
if $developer_sdk_dir == null {
	# <comment copied from bevy example>
	# Assume we're in Xcode, which means we're probably cross-compiling.
	# In this case, we need to add an extra library search path for build scripts and proc-macros,
	# which run on the host instead of the target.
	# (macOS Big Sur does not have linkable libraries in /usr/lib/.)
	$env.LIBRARY_PATH = $"$($env.LIBRARY_PATH)/MacOSX.sdk/usr/lib:$($env.LIBRARY_PATH)"
	print $"Adding '$($env.LIBRARY_PATH)/MacOSX.sdk/usr/lib' to 'LIBRARY_PATH' env variable"
}

# <comment copied from bevy example>
# add homebrew bin path, as it's the most commonly used package manager on macOS
# this is needed for cmake on apple arm processors as it's not available by default
$env.PATH = ($env.PATH | append "/opt/homebrew/bin")

print $env