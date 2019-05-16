#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Build OSX binaries -----------------------------------------------------

cargo build --release
