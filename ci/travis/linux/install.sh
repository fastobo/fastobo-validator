#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Pull musl Linux build container ----------------------------------------

docker pull $IMG_LINUX64
