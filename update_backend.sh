#!/bin/bash

cargo install cargo-progenitor
cargo cargo progenitor -i api/swagger.json -o backend -n backend -v 0.1.0 --include-client true