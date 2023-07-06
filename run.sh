#!/bin/sh

BIN_PATH="$1"
BIN_FOLDER=$(dirname "$BIN_PATH")
BIN_FILE=$(basename "$BIN_PATH")

TARGET="$2"
TARGET_FOLDER="/tmp"

# Compress the binary
tar -czvf "$BIN_FILE.tar.gz"  -C "$BIN_FOLDER" "$BIN_FILE"
# Copy the compressed binary to the remote host
scp "$BIN_FILE.tar.gz" "$TARGET:$TARGET_FOLDER"
# Decompress the binary
ssh "$TARGET" "tar -xzvf \"$TARGET_FOLDER/$BIN_FILE.tar.gz\"" -C "$TARGET_FOLDER"

ssh "$TARGET" "killall $BIN_FILE"

if [ -z "$RUST_LOG" ]; then
    RUST_LOG="info,rocket=warn";
fi

# Run the binary on the remote host via SSH
ssh -t "$TARGET" "RUST_LOG=$RUST_LOG $TARGET_FOLDER/$BIN_FILE"
