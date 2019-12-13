#!/usr/bin/env bash

PATAT_BIN=patat

if ! hash ${PATAT_BIN} &> /dev/null; then
    >&2 echo "ERROR: ${PATAT_BIN} binary not installed"
    >&2 echo "see: https://github.com/jaspervdj/patat/"
    exit 1
fi

${PATAT_BIN} -w presentation.md
