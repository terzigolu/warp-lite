# warp-lite

Lightweight AGPL fork of [Warp Terminal](https://github.com/warpdotdev/warp) — **no AI, no cloud, no telemetry**.

> ⚠️ **Status: alpha / under construction.** The fork is mid-amputation: terminal core works, AI/cloud/onboarding subsystems are being removed phase-by-phase. See [`FORK_NOTICE.md`](FORK_NOTICE.md) for the relationship with upstream Warp, and the [commit history](https://github.com/terzigolu/warp-lite/commits/warp-lite/main) for current state.

## Why

Upstream Warp is excellent, but bundles a large agentic-development surface (Warp AI, Warp Drive, session sharing, ambient agents, computer use, voice input, telemetry, crash reporting) that some users do not want. This fork cuts that surface and ships only the terminal.

Goals, in order:

1. **Local-first.** Zero outbound network calls at idle. No telemetry. No crash uploads. No backend.
2. **Lighter.** Smaller binary, faster cold build, lower idle RAM. Targets after Phase 4: ~90–110 MB binary, ~200 MB idle RAM, ~3–5 min release build (vs. ~200 MB / ~400 MB / ~10 min upstream).
3. **Faithful to the terminal core.** Block model, GPU renderer, shell integrations, Vim mode, editor, LSP, syntax highlighting — all preserved.

What this fork is **not**: a closed-source repackage, an MIT relicense (the AGPL applies and cannot be downgraded), or a project under the Warp Team.

## Status

| Phase | Subsystem | State |
|---|---|---|
| 0 | Default features purge | ✅ Done |
| 1 | Quick-win crate deletions | ✅ Done (3 crates, –524 LOC) |
| 2.2a | Telemetry macros stubbed to no-op | ✅ Done |
| 2.2b | Telemetry call-site sweep | ⏳ Pending |
| 3 | AI / Auth / Onboarding removal | 🚧 In progress |
| 3.6 | Integration test crate removed | ✅ Done (–18 469 LOC) |
| 4 | Cloud subsystem removal | ⏳ Pending |
| 5 | Editor power-feature trim (deferred to v0.2) | ⏳ Future |

Tagging the first usable build as `v0.1.0-lite` once Phase 4 lands.

## Building (macOS)

The fork keeps all of upstream's build prerequisites. On macOS:

1. **Xcode (full)** — not just Command Line Tools. The terminal renderer compiles Metal shaders (`crates/warpui/build.rs`) and `metal` lives only in Xcode.
2. **Metal Toolchain** — Xcode 26+ ships this as a separate component:
   ```sh
   sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
   xcodebuild -downloadComponent MetalToolchain
   ```
3. **git-lfs** — upstream ships some assets via LFS. `brew install git-lfs && git lfs install`, then a fresh clone or `git lfs pull`.
4. **Rust toolchain** — pinned by `rust-toolchain.toml` (currently 1.92.0); `rustup` handles it automatically.

Then:

```sh
cargo check --workspace          # type-check (~2 min cold)
cargo build --release -p app     # release build (target/release/warp-oss)
```

The default binary is `warp-oss` (declared via `default-run` in `app/Cargo.toml`).

## Linux / Windows

Upstream supports both. The fork has not yet been smoke-tested on either; the AI / cloud removal touches platform-agnostic code, so they should keep working, but treat first builds on those platforms as "report bugs and we fix" until tagged v0.1.0-lite.

## Contributing

Issues and PRs welcome. Two ground rules:

1. **License.** Contributions are accepted under AGPL-3.0-only. Do not paste code from non-AGPL/MIT-compatible sources.
2. **Cherry-pick discipline for upstream fixes.** If you want a renderer or shell-integration improvement that landed in `warpdotdev/warp`, port it via `git cherry-pick -x <sha>` from the `upstream-tracking` branch — that preserves AGPL §13 attribution.

## License

- Source code: **AGPL-3.0-only** (see [`LICENSE-AGPL`](LICENSE-AGPL)). Inherited from upstream Warp; cannot be relicensed.
- The two crates `warpui` and `warpui_core` retain their original **MIT** license (see [`LICENSE-MIT`](LICENSE-MIT)), matching upstream.

Trademark "Warp" belongs to Denver Technologies, Inc. — see [`FORK_NOTICE.md`](FORK_NOTICE.md).
