#!/bin/bash

cargo install cargo-progenitor
cargo progenitor -i api/swagger.json -o backend -n backend -v 0.1.0