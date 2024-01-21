#!/usr/bin/env bash

WHL_FILE_NAME="rust_rule_engine-0.1.0-cp312-cp312-macosx_10_12_x86_64.whl"

# Exit on any error
set -e

# Remove old installation
pipenv uninstall rust-rule-engine

# Build new wheel
pipenv run build-whl

# Install new wheel
pipenv install "./packages/$WHL_FILE_NAME"