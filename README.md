# Phazor

**Phazor** — Because writing SPAs in Python *should* be possible.

Phazor is an experimental WebAssembly-powered framework that helps you build single-page applications (SPAs) using Rust today — and eventually Python. It's fast, fun, and just getting started.

## Build & Test (feature/target matrix)

This repo is a Cargo **workspace** with feature-gated crates. Running `cargo build` / `cargo test` at the **workspace root** will try to compile **every** member with **no features** and the **native** target. That often fails because some crates **require explicit features and/or the `wasm32-unknown-unknown` target**.

### Why `cargo test` at root may fail
- Cargo tests **all workspace members** by default.
- Some members (e.g. `phazor_core`, `phazor_web`) are **feature/target gated**.
- If a crate has no defaults, root builds/tests must **opt into the right features/targets** or **exclude** that crate.

### Common commands

**Browser (wasm) dev**
```bash
# Core (wasm target) with IDB store + HTTP sink for the browser
cargo build -p phazor_core --target wasm32-unknown-unknown -F idb-store -F rest-http-wasm

# Web app (served with Trunk from phazor_web/)
cd phazor_web
trunk serve

# Build phazorc (Phazor Compiler)
cargo build -p phazorc

# To run the REST receiver - simulate a real world client - server
cargo run -p phazor_core --example rest_receiver