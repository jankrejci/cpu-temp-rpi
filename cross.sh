#!/bin/sh

docker run \
    -v ".:/project" \
	-v "cargo-dir:/home/pi/.cargo" \
    -v "${SSH_AUTH_SOCK}:/ssh-agent" \
	-it \
	--network="host" \
	rpi-cross-compile-image \
    "$@"
