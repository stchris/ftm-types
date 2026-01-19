#!/usr/bin/env bash

# Temporarily downloads a followthemoney release and copies the yaml schema files

set -euo pipefail;

if [ $# -eq 0 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.2.3"
    exit 1
fi

VERSION=$1

mkdir -p schemas/$VERSION/
curl -sSfL -o v$VERSION.zip https://github.com/opensanctions/followthemoney/archive/refs/tags/v$VERSION.zip
unzip v$VERSION.zip >/dev/null
mv followthemoney-$VERSION/followthemoney/schema/*.yaml schemas/$VERSION/ >/dev/null
rm -rf followthemoney-$VERSION/ v$VERSION.zip