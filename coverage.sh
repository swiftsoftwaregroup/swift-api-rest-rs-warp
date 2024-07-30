#!/usr/bin/env bash

cargo tarpaulin --skip-clean --ignore-tests --out Html --output-dir coverage
