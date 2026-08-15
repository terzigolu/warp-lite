# Warp Lite August 2026 Upstream Sync Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate all material Lite-compatible upstream Warp improvements after `d375729` without regressing Warp Lite's privacy and product boundaries.

**Architecture:** Apply upstream work in independently verifiable slices. Directly cherry-pick isolated retained behavior, surgically port mixed commits, and stage dependency-heavy tab/repository/renderer series as coherent bundles. Maintain an evidence-backed applied/excluded manifest.

**Tech Stack:** Rust workspace, Cargo, WarpUI, macOS Objective-C integration, shell bootstrap scripts, Git cherry-pick provenance.

**Spec:** `docs/superpowers/specs/2026-08-15-warp-lite-upstream-sync-design.md`

## Global Constraints

- Source range is exactly `d3757291a1a1951f4be3f76e0f326a8e3c3dff57..a9c0a1ebda0acfe5e57b6f6df7c6ef744a71f8eb`.
- Default Lite omits `warp_platform`, Warp login, telemetry, outbound Warp cloud clients, and Agent/Oz/cloud/account product UI.
- Terminal view/input/model and local PTY files may receive surgical fixes only; never stub or replace them wholesale.
- Preserve `git cherry-pick -x` provenance or name the source SHA in a surgical-port commit.
- Never launch, terminate, restart, or replace a pre-existing WarpLite/warp-oss process.

---

### Task 1: Baseline and inventory

**Files:**
- Create: `WARP_LITE_SYNC_2026-08.md`
- Modify: none

**Interfaces:**
- Consumes: upstream range and Warp Lite README boundaries.
- Produces: authoritative applied, ported, bundled, and rejected commit lists.

- [ ] Record base/target SHAs and baseline `cargo check -p warp --bin warp-oss` result.
- [ ] Enumerate all 1,143 commits by subject, paths, retained-file coverage, and product-domain exclusions.
- [ ] Seed the report with the security/core candidates already proven absent by `git cherry -v`.
- [ ] Commit the spec, plan, and initial inventory as `docs(warp-lite): plan August upstream sync`.

### Task 2: Security and untrusted-input slice

**Files:**
- Modify: `app/src/notebooks/link.rs`
- Modify: `app/src/terminal/model/terminal_model.rs`
- Modify: retained context-chip and terminal-input files from upstream `4295ec08`
- Modify: `crates/markdown_parser/src/markdown_parser.rs`
- Modify: `app/src/terminal/model/ansi/mod.rs`
- Test: the corresponding retained `*_tests.rs` or inline test modules

**Interfaces:**
- Consumes: upstream commits `7f0c4dd2`, `f3b9ce1c`, `4295ec08`, `c682422f`, and `b9cc454c`.
- Produces: validated handling for malicious Markdown, shell/context input, and OSC output.

- [ ] Apply each commit with `-x` when clean; otherwise stop at the conflict and extract compact conflict context.
- [ ] For each surgical port, run the upstream regression test against the pre-fix state when the current test layout permits, then port the production hunk.
- [ ] Run targeted Markdown, terminal ANSI, terminal model, context-chip, and input tests.
- [ ] Run `cargo check -p warp --bin warp-oss` and commit the slice.

### Task 3: Shell, input, renderer, and macOS slice

**Files:**
- Modify: `app/assets/bundled/bootstrap/zsh_body.sh`
- Modify: retained PTY/input/model/grid files named by selected upstream commits
- Modify: `crates/warpui_core/src/core/*`
- Modify: `crates/warpui/src/platform/mac/*`
- Modify: `crates/warpui/build.rs`
- Test: corresponding shell, WarpUI, editor, and terminal tests

**Interfaces:**
- Consumes: clean candidates `ab081528`, `e59c7a49`, `ae832ff6`, `7a58b59c`, `bf14cbec`, and `7a6044bd`, plus retained conflict candidates such as `f1816928` and `12e455c5`.
- Produces: improved shell correctness, IME behavior, renderer stability, Metal compatibility, Unicode editing, and native-window behavior.

- [ ] Cherry-pick clean candidates in upstream topological order with `-x`.
- [ ] Port conflict candidates one behavior at a time, preserving upstream tests.
- [ ] Run `zsh -n app/assets/bundled/bootstrap/zsh_body.sh` and targeted crate tests.
- [ ] Run default and `warp_platform` Cargo checks and commit the slice.

### Task 4: Files, repository metadata, command palette, and completions slice

**Files:**
- Modify: `crates/repo_metadata/src/*`
- Modify: `app/src/search/command_palette/files/*`
- Modify: retained editor/file-pane and local-control files
- Modify: `Cargo.toml` and `Cargo.lock` only for audited retained dependency upgrades
- Test: repository metadata, file search, editor, and completion test modules

**Interfaces:**
- Consumes: retained fixes including `2aa06b13`, `50853a9b`, `6e192572`, `3bf0899d`, `74a0d675`, `c86f67db`, and compatible command-signature updates.
- Produces: bounded repository work, lower memory/CPU use, correct local file search, and current local completions.

- [ ] Build a dependency order from upstream ancestry and changed-path overlap.
- [ ] Apply the smallest coherent series that contains each selected behavior.
- [ ] Reject any hunk whose only consumer is AI/MCP/cloud indexing and record it.
- [ ] Run targeted tests, default/platform checks, and commit the slice.

### Task 5: Tabs, panes, windows, and local terminal features slice

**Files:**
- Modify: `app/src/workspace/view.rs`
- Modify: `app/src/workspace/view/vertical_tabs.rs`
- Modify: `app/src/workspace/tab_group.rs`
- Modify: `app/src/persistence/sqlite.rs`
- Modify: `crates/persistence/*`
- Modify: retained cross-window drag and pane files
- Test: workspace, persistence, vertical-tab, pane, and window test modules

**Interfaces:**
- Consumes: post-`d375729` tab-group persistence, restore, rename, pinning, drag, multi-pane, and cross-window fixes.
- Produces: a coherent update to the tab-group feature already shipped in Warp Lite.

- [ ] Determine the minimal prerequisite closure for the selected vertical-tab behavior.
- [ ] Apply the series chronologically, excluding shared-session/cloud-only presentation changes.
- [ ] Preserve existing Lite settings and `warp_platform` gates during every conflict.
- [ ] Run targeted workspace/persistence tests, default/platform checks, and commit the slice.

### Task 6: Exhaustive remainder audit and final verification

**Files:**
- Modify: `WARP_LITE_SYNC_2026-08.md`
- Modify: `README.md` only to describe verified shipped source behavior; do not bump or publish a release.

**Interfaces:**
- Consumes: final branch history and the entire upstream source range.
- Produces: requirement-by-requirement evidence that every remaining commit was applied, superseded, bundled, or explicitly rejected.

- [ ] Re-run the upstream inventory and account for every commit in the range.
- [ ] Compare new telemetry, network-client, auth, AI/Oz, and product-surface references against `6658d353`.
- [ ] Run all targeted regressions plus default/platform check and test-compilation gates.
- [ ] Run `git diff --check` and inspect the complete diff/commit provenance.
- [ ] Update the sync report with exact verification outputs and commit the documentation.
