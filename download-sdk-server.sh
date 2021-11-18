#!/usr/bin/env bash
set -e

if [ -z "$1" ] 
  then
    echo "Platform not supplied, expected either of linux, darwin or windows"
    exit 1
fi

which=$1
agones_version="1.18.0"

# unzip doesn't support stdin streams so we need to use a temp file
server_zip=$(mktemp)

curl --fail -L -o "$server_zip" https://github.com/googleforgames/agones/releases/download/v$agones_version/agonessdk-server-$agones_version.zip

# Figure out where .cargo/bin is and put the server there
cargo_bin=$(dirname "$(which cargo)")

if [ "$which" == "windows" ]; then
    ext=".exe"
else
    ext=""
fi

unzip -p "$server_zip" "sdk-server.$which.amd64$ext" > "$cargo_bin/agones-sdk-server$ext"

if [ "$which" != "windows" ]; then
    # Ensure it is executable
    chmod +x "$cargo_bin/agones-sdk-server"

    # Also strip it, as it has >13MiB of debug info for some reason
    strip "$cargo_bin/agones-sdk-server"
fi
