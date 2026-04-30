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

The work is split into compile-driven phases. Each phase commit must keep `cargo check --workspace` green on `warp-lite/main`. **`v0.1.0-lite` is tagged and shipped** — the build is green, telemetry is silenced at runtime, and all Warp Cloud endpoints are blanked. The deeper crate-level removal of AI/Cloud subsystems is targeted at v0.2.

| Phase | Subsystem | State | Notes |
|---|---|---|---|
| **0** | Default features purge | ✅ v0.1 | `agent_mode`, `agent_mode_computer_use`, `agent_onboarding`, `hoa_onboarding_flow`, `gui = ["voice_input"]` removed from defaults. |
| **1** | Quick-win crate deletions | ✅ v0.1 | `managed_secrets_wasm`, `prevent_sleep`, `app-installation-detection` removed (~–524 LOC). |
| **2.2a** | Telemetry macros → no-op | ✅ v0.1 | All `send_telemetry_*!` macros neutralized; runtime emits **0 outbound network calls**. |
| **2.2b** | Telemetry call-site sweep | 🟡 Deferred to v0.2 | Macros are no-op so runtime is safe; physical deletion of ~176 dead call sites is dead-code cleanup. |
| **3** | AI surface stub (compile pass) | ✅ v0.1 | `app/src/search/{ai_context_menu, ai_queries, notebook_embedding}` mods replaced with minimal `warpui`-conformant stubs. `cargo check` green. |
| **3.6** | Integration test crate | ✅ v0.1 | `crates/integration` removed (–18 469 LOC) — e2e tests for cloud/AI flows; useless without backend. |
| **3.x** | AI/Onboarding crate physical removal | ⏳ v0.2 | `crates/ai`, `computer_use`, `onboarding`, `firebase`, `voice_input`, `handlebars`, `warp_js`, `warp_graphql_schema` etc. still in tree but inert at runtime. |
| **4** | Cloud endpoint neutralization | ✅ v0.1 | `server_root_url`, `rtc_server_url`, `firebase_auth_api_key`, `oz_root_url` set to `""`; `crates/graphql` dropped from `default-members`. **0 reachable backends.** |
| **4.x** | Cloud crate physical removal | ⏳ v0.2 | `crates/firebase`, `graphql`, `warp_server_client`, `websocket`, `managed_secrets`, `warp_files` partial — large surface (~5500 LOC, 103 importing files). |
| **5** | Editor power-feature trim | ⏳ Deferred to v0.3+ | `editor` (Zed fork, ~100 K LOC), `lsp`, `node_runtime`, `vim` stay. |

### What `v0.1.0-lite` actually delivers

- ✅ `cargo check --workspace` green on `warp-lite/main`.
- ✅ Telemetry/crash-reporting macros are no-op — **0 outbound network at idle**.
- ✅ Warp Cloud endpoint URLs blanked; even if cloud crates fire, they have nowhere to call.
- ✅ Onboarding/agent-mode default features off.
- ⚠️ AI/Cloud crates still live in the tree (inert). The "lightweight" goal is half-done at the source level — the runtime promise is fully delivered.

The honest summary: **v0.1 is a privacy-respecting Warp**, not yet a slim Warp. Slimness lands in v0.2.

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
cargo check --workspace          # type-check (~2 min cold)
cargo build --release -p app     # release build (target/release/warp-oss)
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
