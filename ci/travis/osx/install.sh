#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Force linking gettext to make envsubst available -----------------------

brew link --force gettext 
