#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Build static x86_64 binaries -------------------------------------------

docker run --rm -it -v $(pwd):/home/rust/src -v $HOME/.cargo:/root/.cargo $IMG_LINUX64 cargo build --release
