# Builds from the parent directory
docker build --progress plain -f Dockerfile ..
# can be `--progress plain` if not logging to satisfaction, or `auth`

# TODO: setup a docker container with incremental compilation