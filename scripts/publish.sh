#!/bin/env bash

cargo readme > README.md
cd diff-utils/
cargo readme > README.md
cd ..
