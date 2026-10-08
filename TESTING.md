# Local verification

Run `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`, `node --test`, `node scripts/check-errors.mjs`, and `stellar contract build`. CLI version: 28.1.0; SDK: 28.0.0. Windows GNU hosts need their existing MinGW linker on PATH. Tests use SDK synthetic addresses and mock authorization; they are not a funded test or an independent audit.

`src/test.rs` checks period boundaries, threshold lifecycle, accounting after each step, group isolation, separate cancellation consent, transfer failure rollback, and missing authorization. `src/error_paths.rs` exercises every error variant through the contract client.

Verified locally on 2026-10-08: fmt check, all-target clippy with warnings denied, all 32 Rust tests, all 9 Node checker tests, 17 error rows in sync, and Stellar CLI 28.1.0 Wasm build pass. Rust used the existing GNU toolchain/MinGW linker and an explicit cached target directory; no contract source or dependencies in SchoolFees were changed. The CLI built this project's normal `target/wasm32v1-none/release/duestreasury.wasm`: 13,072 optimized bytes, nine exported functions, SHA-256 `265b4b71e2d15301226ca6d028a70b2677686e59194959bc8fa6935d7d3b7434`. SDK is pinned to 28.0.0 with Cargo.lock. No deployment, network-funded test or independent audit was performed.
