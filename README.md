# Leptos Trello Clone

An experimental Trello clone built with Rust and WebAssembly.

![Demo](./demo.gif)

## Setup

The app currently runs on nightly version of Rust, see specific version in `rust-toolchain.toml` file.

During first time setup, run `bash setup.sh` to install dependencies (`wasm-bindgen-cli` and `trunk`) and build the project and host it on `localhost:8080`.

Afterwards, running `trunk serve --open` will suffice.

## To Do

- [ ] Reorder cards within a list (column)
- [ ] Adding new lists (columns)
- [ ] Implement proper BE
