#!/bin/bash

CARGO_MANIFEST_DIR=. mdbook build book 2>&1 | tee test-book-err

failures=$( grep '\(Panicked\|Failed\)' test-book-err )

if [[ -n $failures ]]; then
    echo "CI: stderr contained failures"
    rm test-book-err
    exit 1
else
    echo "CI: no failures detected in stderr"
    rm test-book-err
fi
