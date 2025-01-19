#!/usr/bin/env bash

#MISE description="Lint the codebase"

cargo clippy -- -D warnings
