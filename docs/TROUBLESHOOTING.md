# Troubleshooting

## `cargo run` fails in `zed-font-kit` with `E0308` (macOS)

Symptom (example):
- Build fails while compiling `zed-font-kit` and reports `E0308` with a note about multiple versions of `core-graphics` in the dependency graph.

Cause:
- A `core-graphics` version mismatch between `core-text` and `zed-font-kit` can cause type identity conflicts on macOS (e.g., `CGFont` from different crate versions).

Fix (in this repo):
- Zeno pins `core-text` to `=21.0.0` in `Cargo.toml` on macOS to prevent pulling in `core-graphics = 0.25` while GPUI expects `core-graphics = 0.24`.
