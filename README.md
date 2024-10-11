# Readme

Frontend for dxp-todo-backend

- Uses Progenitor client to connect to the backend API.

cargo build --no-default-features --target wasm32-unknown-unknown --features "web"

cargo run

## Recommendation

Install cargo watch to hot-reload the server during development,
either via cargo install or your distros package manager.
