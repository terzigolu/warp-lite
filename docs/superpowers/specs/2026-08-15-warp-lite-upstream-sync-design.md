# Warp Lite August 2026 Upstream Sync Design

## Goal

Bring every materially useful upstream Warp change after `d375729` into Warp Lite when it improves privacy, security, terminal behavior, macOS behavior, local development, editor/file workflows, performance, or the existing tabs/panes feature set, while preserving Warp Lite's local-first product boundary.

## Source and Target

- Source range: `d3757291a1a1951f4be3f76e0f326a8e3c3dff57..a9c0a1ebda0acfe5e57b6f6df7c6ef744a71f8eb`
- Target base: `warp-lite/main` at `6658d353848a8f458b5b87ce30297bf686b1f47f`
- Working branch: `warp-lite/sync-2026-08`
- Provenance: every upstream commit is applied with `git cherry-pick -x` or, when Lite divergence prevents a literal cherry-pick, the resulting commit message records the upstream SHA.

## Non-Negotiable Product Boundaries

Default Warp Lite must continue to:

- omit the positive `warp_platform` feature;
- start without Warp login or account onboarding;
- avoid adding telemetry call sites or outbound Warp cloud clients;
- omit Warp Agent/Oz, billing, pricing, rewards, referrals, teams, Drive, shared-session, and cloud product surfaces;
- preserve the terminal view/input/model, local PTY, renderer, editor, tabs, panes, settings, themes, completions, Markdown, Codex/Claude notifications, and local filesystem workflows;
- leave any pre-existing `/Applications/WarpLite.app` or `warp-oss` process untouched.

Protected terminal files may receive narrow upstream fixes, but they must never be replaced with stubs or wholesale upstream copies.

## Selection Model

Each upstream commit is classified into one of four outcomes:

1. **Apply directly** when its patch is limited to retained Lite behavior and does not add a forbidden surface.
2. **Port surgically** when the behavior belongs in Lite but the upstream patch also touches removed AI/cloud/tests or conflicts with Lite feature gates.
3. **Apply as a dependency bundle** when tabs, panes, persistence, repository metadata, or renderer changes require a coherent prerequisite chain.
4. **Reject with evidence** when the commit is exclusively AI/Oz/cloud/account/telemetry/network, platform-only outside the current macOS release, or depends on a removed crate with no retained consumer.

## Implementation Slices

### Security and untrusted-input handling

Port Markdown link validation, iTerm OSC non-inline file-write blocking, display-chip shell quoting, Markdown delimiter overflow handling, malformed OSC 1337 handling, and applicable dependency advisories. Keep regression tests for PTY-controlled and Markdown-controlled inputs.

### Terminal, shell, input, and renderer

Apply retained PTY write reliability, zsh bootstrap/render fixes, IME cursor updates, wide-character/grid crash fixes, Core Text and glyph fixes, Metal deployment targeting, link detection, shell history/bootstrap, and terminal rendering improvements.

### Editor, files, command palette, and local development

Apply Unicode find/replace, file search correctness, local Markdown refresh, code editor path handling, local CLI/resource fixes, repository watcher performance, and completion updates whose dependency graph remains Lite-compatible.

### Tabs, panes, windows, and macOS

Apply macOS native-window fixes and stage tab-group persistence, rename, pinning, drag, multi-pane, and cross-window changes as a dependency-aware bundle. Do not take cloud/shared-session indicators merely because they share tab code.

### Performance

Apply fixes that reduce retained hot-path work, allocations, clones, runaway watchers, stale work, terminal lag, or renderer overhead. Do not import AI/cloud performance work for code that default Lite does not execute.

## Conflict Policy

- Inspect conflicts one file at a time with `.agents/skills/resolve-merge-conflicts/scripts/extract_conflict_context.py`.
- Preserve Lite `#[cfg(feature = "warp_platform")]` boundaries.
- Prefer the upstream algorithm and tests for retained behavior, adapted to Lite module layout.
- Drop changes only when the affected consumer is intentionally absent; record each dropped hunk in the sync report.
- Abort a cherry-pick rather than resolve by restoring a forbidden product surface.

## Verification

Each slice must pass targeted tests plus `cargo check -p warp --bin warp-oss`. Final verification must include:

- `cargo check -p warp --bin warp-oss`
- `cargo check -p warp --bin warp-oss --features warp_platform`
- test compilation in default and `warp_platform` modes using the repository's required `agent_management_view` test feature where necessary
- targeted regression tests for all manually ported fixes
- `git diff --check`
- an audit of new telemetry/network/auth/AI symbols against the pre-sync base
- review of the final commit inventory and the explicit exclusion report

No live app launch, replacement, restart, or process termination is part of this sync session.
