[private]
@default:
  just --list

build-debug:
  cargo build

build-debug-wasm:
  cargo build --target wasm32-unknown-unknown --features "wasm"

build-release:
  cargo build --target wasm32-unknown-unknown --features "wasm" --release

lint:
  cargo clippy

test-debug:
  cargo test

test-release:
  cargo test --release

test-wasm:
  echo '{ "plugins": ["./target/wasm32-unknown-unknown/release/dprint_plugin_tailwindcss.wasm"] }' >> dprint.test.json
  dprint check --config dprint.test.json
  rm dprint.test.json

# vim: set ts=2 sw=2 sts=2 et:
