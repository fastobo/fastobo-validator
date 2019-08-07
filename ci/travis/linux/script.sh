#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Update Cargo registry and check code -----------------------------------

cargo check

# --- Build static x86_64 binaries -------------------------------------------

docker run --rm -it -v $(pwd):/home/rust/src $IMG_LINUX64 cargo build --release
