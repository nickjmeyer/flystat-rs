#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

rm -f ./.git/hooks/pre-commit*
ln -s ../../tools/git/pre-commit ./.git/hooks/
