# This provides a bunch of environment variables
# including SURREAL_PASS which is the root production password for thd db
source ./env.nu

surreal start file:surreal.db