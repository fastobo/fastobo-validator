#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Generating Bintray deployment configuration ----------------------------

export DATE=$(date -I)
export TAG=${TRAVIS_TAG:-master}
export DESC=$([ -z "$TRAVIS_TAG" ] && echo "Development version built from latest commit on master branch" || echo "Release v$TRAVIS_TAG")


for template in ci/bintray/*.json.in; do
  envsubst < "$template" | tee "${template%.in}"
done
