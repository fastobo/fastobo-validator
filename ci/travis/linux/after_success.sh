#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Generating Bintray deployment configuration ----------------------------

export DATE=$(date -I)
for template in ci/bintray/*.json.in; do
  envsubst < "$template" > "${template%.in}"
done

# --- Strip binaries if this is a release build ------------------------------

if [ ! -z "$TRAVIS_TAG" ]; then
  sudo strip target/x86_64-unknown-linux-musl/release/fastobo-validator || :
fi

# --- Compress all binaries --------------------------------------------------

sudo upx -9 target/x86_64-unknown-linux-musl/release/fastobo-validator || :

# --- Package x86_64-linux-musl ----------------------------------------------

mkdir -p dist
tar czf dist/fastobo_validator-x86_64-linux-musl.tar.gz \
  README.md CHANGELOG.md COPYING \
  -C target/x86_64-unknown-linux-musl/release/ fastobo-validator
