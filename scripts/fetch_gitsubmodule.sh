#!/bin/bash
set -uex

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
REPO_DIR=$DIR/"../.."
cd $REPO_DIR

git submodule update --init --recursive
git pull --recurse-submodules
