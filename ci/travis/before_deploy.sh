#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Generating Bintray deployment configuration ----------------------------

export DATE=$(date -I)
for template in ci/bintray/*.json.in; do
  envsubst < "$template" | tee "${template%.in}"
done

# --- Strip binaries if this is a release build ------------------------------

if [ ! -z "$TRAVIS_TAG" ]; then
  for bin in target/*/release/fastobo-validator; do
    strip $bin
  done
fi

# --- Compress all binaries --------------------------------------------------

for bin in target/*/release/fastobo-validator; do
  sudo upx -9 $bin
done

# --- Package x86_64-linux-musl ----------------------------------------------

mkdir -p $TRAVIS_BUILD_DIR/dist
tar czf dist/fastobo_validator-x86_64-linux-musl.tar.gz -C target/x86_64-unknown-linux-musl/release/ fastobo-validator
