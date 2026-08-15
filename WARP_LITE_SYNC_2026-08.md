# Warp Lite upstream sync — August 15, 2026

## Outcome

We reviewed 1,143 upstream commits between Warp Lite's previous sync base,
`6658d353848a8f458b5b87ce30297bf686b1f47f` (`warp-lite/main`), and the audited
upstream snapshot, `a9c0a1ebda0acfe5e57b6f6df7c6ef744a71f8eb`
(`upstream/master`).

The `warp-lite/sync-2026-08` branch received the changes that fit the Warp Lite
product boundary: security and privacy fixes, terminal reliability, editor and
file experience improvements, performance work, macOS/Windows/WSL/SSH
compatibility fixes, and developer-tooling improvements. At the audit snapshot,
the branch contained 195 commits: 193 sync/adaptation commits, one design and
implementation plan, and this report. The audit diff covered 286 files; before
the report was added, the code diff covered 285 files with 20,633 insertions and
2,530 deletions.

Eligible changes were cherry-picked directly. Useful changes that depended on
upstream APIs removed from Warp Lite, or on Warp-platform-only APIs, were ported
as small local adaptations. The exact audited commit manifest is available with:

```bash
git log --reverse --oneline v0.5.6-lite..0b42c9e18
```

## Major changes included

### Security and privacy

- Fixed vulnerabilities and unsafe behavior around Markdown link opening,
  iTerm file downloads, display-chip RCE, code-search command injection, SSH
  command injection, and repository-directory shell escaping (`1f7460146`,
  `6ad5d2c8c`, `6cd0d52f0`, `c3fa1004e`, `f0ddba1d3`, `14aa3164b`,
  `7f85ae753`).
- Stripped environment assignments before command-blocklist checks
  (`c9e9f01a5`), redacted authentication URLs in logs (`21c810b96`), and
  preserved the MCP secret-redaction preference (`1a9d95801`).
- Added a user-controlled setting for OSC 52 clipboard reads and a notification
  when a read is blocked (`07db02cff`, `ed60747d3`).
- Fixed an H2 debug panic and adapted the lockfile (`758591ca6`, `1f2f6f96b`),
  and updated `serde_with` for its security advisory (`cca0b9248`).

### Terminal, shell, and remote-session reliability

- Fixed response-sequence loss during blocked PTY writes, zsh grid corruption,
  wrapped-line path detection, repeated multiline command prefixes,
  wide-character resize crashes, and startup inline-image rendering.
- Enabled OSC 8 hyperlinks by default and fixed both a missing-parameter OSC
  1337 panic and an empty OSC hyperlink edge case.
- Hardened shell PATH capture, quoted home/worktree paths, PowerShell history
  and bootstrap behavior, zsh explicit-width prompts, and SSH wrapper and
  `RemoteCommand` behavior.
- Generator commands now run in their own process groups and are terminated
  together with their child processes on cancellation (`a5365a37a`,
  `955171a88`). Oversized PTY environments now fail fast before `E2BIG`
  (`9483c9c4f`).
- Remote-writer shutdown errors were demoted from crash-report severity to
  normal logs, and shell exit reasons were added to diagnostic logs.

### Editor, files, and Vim

- Fixed non-ASCII find/replace, Markdown syntax highlighting, HTML comment
  hiding, header/table selection, local Markdown image refresh, and viewer
  preferences.
- Added `Copy file path`, path and URI copying, directory exclusion in
  file-only searches, and natural numeric sorting in the file tree.
- Adapted the text-editor autosave setting to Warp Lite's settings and event
  APIs.
- Expanded Vim behavior with count + `gg`, visual paste, `d%`/`c%`/`y%`,
  indent/dedent, Vim mode in the environment-variable editor, find refocus,
  and soft-wrapped visual-line movement.

### Tabs, groups, and desktop UX

- Integrated the horizontal/vertical tab grouping and pinning chain, including
  persistence, cross-window drag behavior, pin/group invariants, colors,
  rename/collapse actions, multi-pane headers, and crash fixes.
- Fixed fullscreen corners, title-bar search hiding, tab backgrounds and
  contrast, active-tab color cycling, suggestion bounds in small windows, and
  session-menu maximum height.
- Fixed first-open focus for Quake windows, Dock visibility for dedicated
  hotkey windows, and native macOS window chrome and zoom behavior.
- Stopped Warp from automatically claiming common file types by setting
  `LSHandlerRank=Alternate`.

### Performance, platform compatibility, and developer experience

- Shared gitignore rules across events, skipped Git work for filesystem-only
  watchers, stopped watching directory symlink targets, and reduced unnecessary
  path canonicalization and ignored-directory rebuilds.
- Merged identical style runs before Core Text shaping and rendered box-drawing
  glyphs procedurally.
- Removed full process-table CPU scans for every Windows session, recognized
  WSL UNC hosts case-insensitively, and allowlisted Intel Xe adapters affected
  by old Mesa versions.
- Improved macOS bootstrap behavior with headless/non-interactive Homebrew,
  transient retries, codesign timestamp retries, verified `cargo-binstall`,
  and installed-tool binary verification.
- Fixed path handling in `app/build.rs` and target-directory resolution in the
  build scripts.

## Warp Lite boundary and excluded work

The batch audit excluded changes that introduced or expanded AI/agent behavior,
cloud account/object flows, billing, teams, orchestration, remote control, or
new telemetry collection and delivery. Representative upstream commits that
were intentionally left out include:

- `e367c9de`: queued-prompt and AI behavior.
- `912e4540`: AI Markdown flow.
- `9d3f3e1e`: cloud/AI-dependent change.
- `63b582890`: agent SDK.
- `a1af68cbd`: MCP JSON viewer component absent from Lite.
- `43c21508`: lifecycle architecture absent from Lite.
- `d4c4cf9b`: `warpctrl` infrastructure absent from Lite.
- `af29c593b`: Copy Current Path change coupled to an action absent from Lite.
- `89f742fa`: managed-secrets/platform boundary.
- `d56d70ade`: change coupled to a schema removed from Lite.

The default-feature diff adds only:

```text
osc_hyperlinks
grouped_tabs
pinned_tabs
```

The diff adds no new `send_telemetry`, `send_event`, `report_event`,
`track_event`, `emit_telemetry`, or `log_telemetry` call sites. Changes under AI
directories are limited to security hardening in existing/dormant source and
local Markdown image refresh support; no AI or agent feature was enabled.

## Validation

| Check | Result |
| --- | --- |
| `cargo check -p warp --bin warp-oss` | Passed |
| `cargo check -p warp --bin warp-oss --features warp_platform` | Passed |
| `cargo test -p repo_metadata` | 48 passed, 0 failed, 3 ignored |
| `cargo test -p warp_util` | 64 unit tests and 3 doctests passed |
| Focused WSL UNC tests | 2/2 passed |
| `cargo test -p warp_terminal` | 111 passed, 0 failed, 2 ignored |
| Focused terminal escape tests | 22/22 passed |
| `cargo test -p vim` | 68/68 passed |
| `cargo test -p markdown_parser` | 151/151 passed |
| `cargo test -p warpui_core` | 288 passed, 0 failed, 7 ignored; 2 doctests passed, 1 ignored |
| Warp library test binary with `--features agent_management_view` | Compiled successfully |
| Local command-executor process-group tests with `agent_management_view,local_tty` | 3/3 passed |
| `zsh -n` / `bash -n` bootstrap script checks | Passed |
| `git diff --check` | Passed |
| Conflict-marker scan | Clean |

The build emits many existing unused/dead-code and unexpected-`cfg` warnings;
none of them are errors.

## Known validation limits

- The default `cargo test -p warp --lib --no-run` still fails because two test
  references in `app/src/workspace/view_test.rs` access the
  `agent_management_view` field without a matching feature guard. The same test
  binary compiles with `--features agent_management_view`; this test-fixture
  mismatch predates the sync.
- `cargo fmt --all` cannot start because the repository references two missing,
  disabled test modules:
  `app/src/search/ai_context_menu/_disabled_subtree/blocks.rs` and
  `crates/ai/src/agent/action_result/convert_tests.rs`. `git diff --check` is
  clean for the applied changes.
- The Intel Xe/Mesa allowlist test is excluded by platform `cfg` on macOS. Its
  source passes compilation, but it was not exercised on real Linux/Intel
  hardware.
- The installed `/Applications/WarpLite.app` and its running process were
  preserved. The newly packaged release bundle was not launched automatically.

## Release status

The sync and release-preparation work were merged into `warp-lite/main` at
`2f02dca80acaa63ecfa4cd8ae98c76078aeb3c62`. The final release diff from
`v0.5.6-lite` covers 288 files with 20,827 insertions and 2,537 deletions across
197 commits reachable beyond the previous tag.

`v0.5.7-lite` was published as the latest GitHub release with fresh
`WarpLite.dmg` and `WarpLite.app.zip` assets. The optimized build, strict ad-hoc
codesign verification, DMG verification and read-only mount, mounted bundle
metadata, zip integrity, and Mach-O UUID checks all passed. The package remains
ad-hoc signed and is not Apple-notarized.

Settings UI candidates discovered in the follow-up audit after this sync were
not included in `v0.5.7-lite`; they remain candidates for a later patch release.
