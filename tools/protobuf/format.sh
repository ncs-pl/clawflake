#!/usr/bin/env sh
DEPS=$(find api -type f -name "*.proto")
clang-format -i --sort-includes "$DEPS"