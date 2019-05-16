#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Generating Bintray deployment configuration ----------------------------

export DATE=$(date -I)
for template in ci/bintray/*.json.in; do
  envsubst < "$template" > "${template%.in}"
done

# --- Strip binaries if this is a release build ------------------------------

if [ ! -z "$TRAVIS_TAG" ]; then
  strip target/release/fastobo-validator || :
fi

# --- Compress all binaries --------------------------------------------------

upx -9 target/release/fastobo-validator

# --- Package x86_64-apple-darwin --------------------------------------------

mkdir -p $TRAVIS_BUILD_DIR/dist
tar czf dist/fastobo_validator-x86_64-apple-darwin.tar.gz -C target/release/ fastobo-validator
