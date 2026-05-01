# warp-lite

Lightweight AGPL fork of [Warp Terminal](https://github.com/warpdotdev/warp) — **no AI, no cloud, no telemetry**.

> ⚠️ **Status: alpha / under construction.** The fork is mid-amputation: terminal core works, AI / cloud / onboarding subsystems are being removed phase-by-phase. The `warp-lite/main` branch is always green and pushable; in-progress work lives on `phase3-wip` until it compiles. See [`FORK_NOTICE.md`](FORK_NOTICE.md) for the relationship with upstream Warp, and the [commit history](https://github.com/terzigolu/warp-lite/commits/warp-lite/main) for the current shipped state.

## Why

Upstream Warp is excellent, but bundles a large agentic-development surface (Warp AI, Warp Drive, session sharing, ambient agents, computer use, voice input, telemetry, crash reporting) that some users do not want. This fork cuts that surface and ships only the terminal.

Goals, in order:

1. **Local-first.** Zero outbound network calls at idle. No telemetry. No crash uploads. No backend.
2. **Lighter.** Smaller binary, faster cold build, lower idle RAM.
3. **Faithful to the terminal core.** Block model, GPU renderer, shell integrations, Vim mode, editor, LSP, syntax highlighting — all preserved.

What this fork is **not**: a closed-source repackage, an MIT relicense (the AGPL applies and cannot be downgraded), or a project under the Warp Team.

## Roadmap & Status

The work is split into compile-driven phases. Each phase commit must keep `cargo check --workspace` green on `warp-lite/main`. **`v0.1.0-lite` and `v0.2.0-lite` are tagged and shipped.** v0.2 lands the runtime privacy + measurable source-level slimming; full crate-level removal of `ai` (22 K LOC) and `onboarding` (11 K LOC) is targeted at v0.3.

| Phase | Subsystem | State | Notes |
|---|---|---|---|
| **0** | Default features purge | ✅ v0.1 | `agent_mode`, `agent_mode_computer_use`, `agent_onboarding`, `hoa_onboarding_flow`, `gui = ["voice_input"]` removed from defaults. |
| **1** | Quick-win crate deletions | ✅ v0.1 | `managed_secrets_wasm`, `prevent_sleep`, `app-installation-detection` removed (~–524 LOC). |
| **2.2a** | Telemetry macros → no-op | ✅ v0.1 | All `send_telemetry_*!` macros neutralized at the macro layer. |
| **2.2b** | Telemetry call-site sweep | ✅ v0.2 | **1 026 `send_telemetry_*!()` call sites physically deleted** across 168 files; macro definitions removed. |
| **3 (compile pass)** | AI surface stub | ✅ v0.1 | `app/src/search/{ai_context_menu, ai_queries, notebook_embedding}` mods stubbed. |
| **3.6** | Integration test crate | ✅ v0.1 | `crates/integration` removed (–18 469 LOC). |
| **3.x (partial)** | Niche AI crate physical removal | ✅ v0.2 | `crates/voice_input`, `handlebars`, `warp_js`, `warp_graphql_schema` deleted (~–1.5 K Rust LOC + assets). `crates/onboarding` decoupled from `ai` (`LLMId` inlined). |
| **3.x (computer_use)** | AI peripheral gut | ✅ v0.2 | `crates/computer_use` shrunk from 4 K LOC → 200 LOC inert stub. Public API (Action, Screenshot, Vector2I, Actor trait) preserved; runtime is no-op. |
| **3.x (final)** | `crates/ai`, `crates/onboarding` physical removal | ⏳ v0.3 | `ai` (22 K LOC, 172 import paths) + `onboarding` (11 K LOC, 22 deep consumers). Onboarding is now ai-independent so a future drop is unblocked. |
| **4 (network silence)** | Cloud endpoint neutralization | ✅ v0.1 | `server_root_url`, `rtc_server_url`, `firebase_auth_api_key`, `oz_root_url` set to RFC-2606 invalid TLDs; **0 reachable backends.** |
| **4.x (transport)** | Cloud HTTP/WS short-circuit | ✅ v0.2 | `firebase` 145→55 LOC, `websocket` 1080→380 LOC (proxy.rs purged + `connect()` returns Err), `warp_graphql` HTTP transport short-circuited at `client.rs::send_graphql_request`. **0 outbound HTTPS / WS dial.** |
| **4.x (final)** | Full cloud crate `rm -rf` | ⏳ v0.3 | `firebase`, `graphql`, `warp_server_client`, `managed_secrets`, `warp_files` (partial) — blocked by AI crate's deep cloud type usage; unlocks once `crates/ai` goes. |
| **5** | Editor power-feature trim | ⏳ Deferred to v0.4+ | `editor` (Zed fork, ~100 K LOC), `lsp`, `node_runtime`, `vim` stay. |

### What v0.2.0-lite actually delivers

- ✅ `cargo check --workspace` green on `warp-lite/main` (55 crates, down from 63).
- ✅ Telemetry physically removed: 1 026 call sites across 168 files, plus the macro definitions themselves.
- ✅ All cloud endpoint URLs blanked + transport-layer short-circuit (no outbound HTTPS GraphQL, no outbound WS, no Firebase REST). **Idle network call count: 0.**
- ✅ 4 AI/network crates fully deleted (`voice_input`, `handlebars`, `warp_js`, `warp_graphql_schema`).
- ✅ `computer_use` reduced 95 % to inert stub (4 K → 200 LOC).
- ✅ `onboarding` decoupled from `ai`, ready for removal in v0.3.
- ⚠️ `crates/ai` (22 K LOC) and `crates/onboarding` (11 K LOC) and the cloud type-stack still live in the tree — runtime-inert but on disk. v0.3 is the cleanup release.

The honest summary: **v0.2 is a privacy-respecting Warp with most of the bulk gutted at runtime.** True file-level slimness for the two giant crates lands in v0.3.

## What's been removed (`phase3-wip` branch — pending green compile)

### Crates deleted (12)
`ai`, `computer_use`, `onboarding`, `firebase`, `voice_input`, `handlebars`, `warp_js`, `managed_secrets`, `graphql` (warp_graphql), `warp_graphql_schema`, `warp_server_client`, `websocket`.

Plus already on `warp-lite/main`: `integration`, `managed_secrets_wasm`, `prevent_sleep`, `app-installation-detection`.

### App modules deleted (17+)
`app/src/ai/`, `ai_assistant/`, `auth/`, `billing/`, `drive/`, `pricing/`, `cloud_object/`, `code_review/`, `chip_configurator/`, `context_chips/`, `coding_entrypoints/`, `coding_panel_enablement_state/`, `prompt/`, `referral_theme_status/`, `reward_view/`, `server/`, `voice/`, `voltron/`, `session_management/`.

### Surprises documented along the way
- `warp_completer` looked AI-coupled but is actually pure shell completion (history / path) — kept.
- `crates/persistence` carries an AI proto schema (AgentConversation, ModelTokenUsage) baked into local SQLite — surgical deletion required.
- `app/src/settings/onboarding` is an in-app submodule, distinct from the `crates/onboarding` workspace crate. Easy to confuse.

## Estimated wins after `v0.1.0-lite`

| Metric | Upstream baseline | warp-lite target |
|---|---|---|
| Release binary | 180–220 MB | **~90–110 MB** (≈ –60%) |
| Cold `cargo build --release` | 8–12 min | **3–5 min** |
| Idle RAM | 350–450 MB | **~200–300 MB** |
| LOC compiled | ~750 K | **~480 K** (–36%) |
| Outbound network calls at idle | telemetry + Sentry + RTC | **0** |

## Branch structure

```
origin/warp-lite/main      ← default; always green; cherry-pick target for upstream fixes
origin/phase3-wip          ← Phase 3 in-flight (cargo check still red, agent-driven)
origin/upstream-tracking   ← weekly mirror of warpdotdev/warp@master (orphan, never merged)
upstream/master            ← read-only; cherry-pick source via `git cherry-pick -x <sha>`
```

Two tags worth knowing during the Phase 3 work:

- `phase3-progress` — checkpoint after `lib.rs` + `persistence/sqlite.rs` cleared (commit `c218492`).
- `phase3-progress-2` — checkpoint after the mass cfg-gate pass on `terminal/view.rs` and `workspace/view.rs` (commit `6be4a03`).

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
cargo check --workspace                    # type-check (~2 min cold)
cargo build --release --bin warp-oss       # release build (target/release/warp-oss)
```

The default binary is `warp-oss` (declared via `default-run` in `app/Cargo.toml`).

## Linux / Windows

Upstream supports both. The fork has not yet been smoke-tested on either; the AI / cloud removal touches mostly platform-agnostic code, so they should keep working, but treat first builds on those platforms as "report bugs and we fix" until tagged v0.1.0-lite.

## Contributing

Issues and PRs welcome. Two ground rules:

1. **License.** Contributions are accepted under AGPL-3.0-only. Do not paste code from non-AGPL/MIT-compatible sources.
2. **Cherry-pick discipline for upstream fixes.** If you want a renderer or shell-integration improvement that landed in `warpdotdev/warp`, port it via `git cherry-pick -x <sha>` from the `upstream-tracking` branch — that preserves AGPL §13 attribution.

## License

- Source code: **AGPL-3.0-only** (see [`LICENSE-AGPL`](LICENSE-AGPL)). Inherited from upstream Warp; cannot be relicensed.
- The two crates `warpui` and `warpui_core` retain their original **MIT** license (see [`LICENSE-MIT`](LICENSE-MIT)), matching upstream.

Trademark "Warp" belongs to Denver Technologies, Inc. — see [`FORK_NOTICE.md`](FORK_NOTICE.md).
