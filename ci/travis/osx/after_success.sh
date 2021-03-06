#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Generating Bintray deployment configuration ----------------------------

export DATE=$(date "+%Y-%m-%d")
for template in ci/bintray/*.json.in; do
  envsubst < "$template" > "${template%.in}"
done

# --- Strip binaries if this is a release build ------------------------------

if [ ! -z "$TRAVIS_TAG" ]; then
  strip target/release/fastobo-validator || :
fi

# --- Compress all binaries --------------------------------------------------

upx -9 target/release/fastobo-validator || :

# --- Package x86_64-apple-darwin --------------------------------------------

cd $TRAVIS_BUILD_DIR
mkdir -p dist
tar czf dist/fastobo_validator-x86_64-apple-darwin.tar.gz \
  README.md CHANGELOG.md COPYING \
  -C target/release/ fastobo-validator
