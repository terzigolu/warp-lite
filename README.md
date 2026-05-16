# warp-lite

Lightweight AGPL fork of [Warp Terminal](https://github.com/warpdotdev/warp), focused on a local-first terminal experience without Warp account login, bundled AI workflows, cloud onboarding, or telemetry as a product requirement.

> Status: alpha, but usable on macOS. The latest shipped build is **v0.5.3-lite**. It builds, launches, and is published with downloadable `WarpLite.dmg` and `WarpLite.app.zip` assets on GitHub Releases.

Latest release:

- [v0.5.3-lite](https://github.com/terzigolu/warp-lite/releases/tag/v0.5.3-lite)
- Tag: `v0.5.3-lite`
- macOS artifacts: `WarpLite.dmg` (~124 MB), `WarpLite.app.zip` (~114 MB)

See [`FORK_NOTICE.md`](FORK_NOTICE.md) for the relationship with upstream Warp.

## Why

Upstream Warp is excellent, but includes a large agentic-development and cloud surface that some users do not want in their terminal. This fork keeps the terminal core and progressively removes or disables the product surfaces around AI, cloud sync, billing, onboarding, telemetry, and account login.

Goals, in order:

1. **Local-first.** No Warp account required. No login gate for opening a terminal.
2. **Terminal-first.** Preserve the block terminal, GPU renderer, shell integrations, tabs, panes, settings, themes, command palette, editor basics, completions, and markdown rendering.
3. **Lighter over time.** Remove AI/cloud code paths carefully without breaking terminal rendering or input.
4. **Honest status.** Some source modules are still present while the default build avoids their product paths. This README tracks that split explicitly.

What this fork is **not**: a closed-source repackage, an MIT relicense, or a project maintained by the Warp Team. The AGPL applies and cannot be downgraded.

## Install

Download the latest macOS build from:

```text
https://github.com/terzigolu/warp-lite/releases/latest
```

Use `WarpLite.dmg`, then drag `WarpLite.app` into `/Applications`.

The packaged app uses:

- Bundle identifier: `dev.warp-lite.WarpLite`
- App name: `WarpLite`
- Current bundle version: `0.5.3-lite`

## Current Shipped State

The current release is `v0.5.3-lite`.

| Area | State | Notes |
|---|---|---|
| Terminal core | Works | Core terminal view/input/model files are preserved. Do not wholesale stub them. |
| macOS app bundle | Works | `script/build-warp-lite-app.sh` builds `WarpLite.app`. |
| DMG release | Works | `WarpLite.dmg` is published in GitHub Releases. |
| Warp login gate | Disabled | `skip_firebase_anonymous_user` is enabled by default. Startup, "skip login", and visible account/billing menu entry points are hardened away from Warp auth in the lite build. |
| Telemetry product goal | Removed/neutralized | Historical telemetry call-site cleanup is part of the fork; keep auditing before claiming perfect network silence. |
| Context Panel / Tools Panel | Removed from shipped UI | The experimental Context Panel was deleted from the app wiring in `v0.5.1-lite` after causing instability and stale data issues. |
| Codex / Claude Code notifications | Kept | These are intentionally preserved for the lite fork. |
| Markdown viewer | Kept | `markdown_tables` and `markdown_mermaid` remain in defaults. |
| Agent mode | Not a target | Agent-mode product surfaces should stay out of the lite app. |

## What Changed Recently

### v0.5.2-lite

- Restored `skip_firebase_anonymous_user` in default features.
- Fixed the regression where the welcome/sign-up modal still appeared and "Skip for now" attempted Warp/Firebase auth.
- Rebuilt and published fresh `WarpLite.dmg` and `WarpLite.app.zip` release assets.

### v0.5.3-lite

- Refreshed this README to match the real v0.5.2-lite state.
- Hardened the no-login path so the compiled lite feature bypasses auth onboarding even if runtime flags drift.
- Hid or no-op'd remaining visible sign-up, upgrade, referral, logout, and anonymous-user menu/actions in the lite build.

### v0.5.1-lite

- Slimmed default features.
- Removed the experimental Context Panel source/wiring from the shipped app path.
- Gated additional agent/cloud management UI surfaces.

### v0.5.0-lite and earlier

- Gutted large parts of codebase indexing and AI-adjacent background work.
- Deleted or stubbed several AI/cloud peripheral crates.
- Removed large telemetry call-site surface from earlier phases.
- Preserved the terminal renderer/input stack after a failed over-aggressive stub attempt proved that compile success is not enough.

## Removed From Source

These crates or app modules are no longer present in the current tree:

| Path | Status |
|---|---|
| `crates/integration` | Removed |
| `crates/firebase` | Removed |
| `crates/voice_input` | Removed |
| `crates/handlebars` | Removed |
| `crates/warp_js` | Removed |
| `crates/warp_graphql` | Removed |
| `crates/warp_graphql_schema` | Removed |
| `crates/command-signatures-v2` | Removed |
| `crates/serve-wasm` | Removed |
| `crates/prevent_sleep` | Removed |
| `crates/managed_secrets_wasm` | Removed |
| `crates/app-installation-detection` | Removed |
| `app/src/onboarding` | Removed |

## Still Present And Needs Work

These modules still exist and should be treated as the next cleanup targets. Some are default-disabled, partially stubbed, or unreachable in normal lite flows, but they are not physically gone.

| Path | Why it matters | Suggested next move |
|---|---|---|
| `app/src/auth` | Login UI and auth flow still exist in source. Startup/skip/menu paths are hardened in the lite build, but the module is not physically gone. | Continue shrinking or feature-gating auth UI internals after verifying shared `AuthStateProvider` consumers. |
| `app/src/ai` | Large AI UI/product surface remains. | Continue surgical feature-gating and deletion; avoid terminal core wholesale stubs. |
| `crates/ai` | Still a major compiled/source dependency. | Continue reducing agent/indexing/ambient modules behind stable APIs. |
| `crates/onboarding` | Still present even though app onboarding module is gone. | Finish crate-level cleanup if consumers are gone or can be stubbed safely. |
| `app/src/billing` | Billing UI should not ship in a local-first lite terminal. | Gate/remove visible and reachable billing flows. |
| `app/src/voice` | Voice feature source remains although `crates/voice_input` is gone. | Remove dead app-side voice surfaces or gate them out. |
| `crates/websocket` | Network transport crate still exists. | Verify consumers, then stub or delete if no terminal feature needs it. |
| `crates/warp_server_client` | Warp backend client remains. | Audit call sites and remove once auth/cloud dependencies are gone. |
| `crates/managed_secrets` | Cloud/secret product surface remains as a stub candidate. | Keep API only if required, otherwise delete. |
| `crates/warp_files` | Cloud/file integration residue. | Audit dependencies before removal. |

## Guardrails

The terminal works because its core was preserved. Keep these files off any broad deletion or wholesale-stub plan:

- `app/src/terminal/view.rs`
- `app/src/terminal/input.rs`
- `app/src/terminal/block_list_element.rs`
- `app/src/terminal/view/`
- `app/src/terminal/input/`
- `app/src/terminal/local_tty/terminal_manager.rs`
- `app/src/terminal/alt_screen/alt_screen_element.rs`
- `app/src/terminal/model/`

If a change makes the build green by replacing terminal rendering/input/model code with small stubs, that change is wrong for warp-lite. Verify with launch testing, not just `cargo check`.

## Build

The fork keeps upstream's macOS build prerequisites:

1. Full Xcode, not just Command Line Tools.
2. Metal Toolchain:

   ```sh
   sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
   xcodebuild -downloadComponent MetalToolchain
   ```

3. git-lfs:

   ```sh
   brew install git-lfs
   git lfs install
   git lfs pull
   ```

4. Rust toolchain pinned by `rust-toolchain.toml`.

Common commands:

```sh
cargo check -p warp --bin warp-oss
CARGO_BUILD_JOBS=4 cargo build --release --bin warp-oss
script/build-warp-lite-app.sh
rm -f WarpLite.dmg
hdiutil create -volname WarpLite -srcfolder WarpLite.app -ov -format UDZO WarpLite.dmg
```

Verification used for the latest release:

```sh
cargo check -p warp --bin warp-oss
CARGO_BUILD_JOBS=4 cargo build --release --bin warp-oss
codesign --verify --deep --strict --verbose=2 WarpLite.app
hdiutil verify WarpLite.dmg
```

Known caveat: full `cargo fmt --check` can currently fail because the repository still references disabled/removed upstream files. Prefer targeted formatting/checks until that cleanup is complete.

## Branch Structure

```text
origin/warp-lite/main      default branch; current shipped work
origin/upstream-tracking   read-only mirror/cherry-pick source for upstream Warp changes
upstream/master            upstream Warp source
```

Historical phase branches and tags may still exist, but the public state should be read from `warp-lite/main`, the tags, and the GitHub Releases page.

## Release History

| Tag | Summary |
|---|---|
| `v0.5.3-lite` | README refresh, no-login hardening, and remaining visible account/upsell action cleanup. |
| `v0.5.2-lite` | No-login hotfix; fresh DMG/app zip assets. |
| `v0.5.1-lite` | Default feature diet and Context Panel removal from shipped app path. |
| `v0.5.0-lite` | AI/codebase-index cleanup and bundle version bump. |
| `v0.4.0-lite` | Managed secrets/onboarding reduction work. |
| `v0.3.x-lite` | Context Panel experiments; later removed from shipped path. |
| `v0.2.x-lite` | Telemetry call-site cleanup, UI hiding, niche crate removals. |
| `v0.1.0-lite` | Initial default feature purge and first green lite build. |

## License

- Source code: **AGPL-3.0-only** (see [`LICENSE-AGPL`](LICENSE-AGPL)).
- `warpui` and `warpui_core` retain their original **MIT** license (see [`LICENSE-MIT`](LICENSE-MIT)).

Trademark "Warp" belongs to Denver Technologies, Inc. See [`FORK_NOTICE.md`](FORK_NOTICE.md).
