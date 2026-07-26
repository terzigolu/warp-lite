# Warp Lite Performance and Source Slimming

## Summary

Warp Lite should remain the same dependable block-based terminal while becoming measurably smaller, faster to start, cheaper to keep idle, and easier to build. The work must remove AI, account, cloud, and Warp-backend product weight without weakening the terminal renderer, input handling, shell integration, panes, tabs, command palette, or local editing experience.

## Problem

The shipped application already avoids login, telemetry, and normal idle network activity, but large disabled product areas remain in the source tree and dependency graph. Previous broad deletion attempts proved that a green compile is insufficient: replacing terminal-core code with stubs can produce an application that launches without rendering a usable window.

## Goals

- Make every "lite" claim measurable and reproducible.
- Exclude unused AI/cloud/account/backend code from the release build, then remove code that is demonstrably unreachable.
- Improve startup, idle resource use, binary size, and build cost without regressing terminal behavior.
- Keep privacy guarantees and vetted upstream terminal improvements.

## Non-goals

- Rewriting the terminal renderer or input model.
- Removing local-first power features solely to minimize line count.
- Reintroducing Warp services, login, telemetry, or cloud sync.
- Claiming performance gains based only on deleted source lines.

## Behavior

1. A normal Warp Lite release opens directly into a usable local terminal without requiring or offering a Warp account login.

2. Blocks, panes, tabs, splits, command editing, shell integration, command history, command search, themes, local file navigation, and supported local editor behavior continue to work as they did before the slimming work.

3. The release performs no telemetry, AI-provider, Warp GraphQL, Warp backend, Firebase, cloud-drive, shared-session, billing, or account network request during startup or idle use.

4. AI agent controls, AI context controls, account, upgrade, billing, referral, cloud-drive, and cloud-workspace surfaces do not appear in the lite application.

5. The lite release does not include disabled AI/cloud/account/backend implementation merely because dormant code still compiles. Components outside the lite product contract must be excluded from the release dependency graph before their source is deleted.

6. Every slimming change records comparable measurements from the same benchmark command:
   - Git revision and dirty state.
   - Release binary and application-bundle size when those artifacts exist.
   - Workspace package-manifest count.
   - Presence of forbidden direct application dependencies in the repository's
     current inline Cargo syntax. The release gate uses the resolved Cargo graph.
   - Source-size indicators for the remaining AI/cloud/account product areas.

7. A missing release binary or application bundle does not make the benchmark fail. The report identifies the artifact as missing and tells the developer which build step is required.

8. Benchmark output is deterministic enough to compare two revisions. Machine-specific values are clearly separated from repository and artifact values.

9. Runtime performance claims require measurements from the native macOS application, not only `cargo check`, a browser surface, source line counts, or an unbundled unit test.

10. A runtime benchmark never interrupts an existing Warp Lite session. If the
    selected bundle executable is already running, the harness refuses to launch
    another instance. It never uses broad process termination such as `killall`
    or `pkill`, and cleanup may signal only the exact process started by that
    harness invocation.

11. Runtime comparison covers, at minimum:
    - Cold launch to a usable terminal.
    - Warm launch.
    - Idle resident memory and CPU after a fixed settling period.
    - First shell-prompt readiness.
    - Sustained output and large-scrollback responsiveness.

12. A performance optimization is accepted only when its target metric improves or remains within the documented tolerance and the terminal smoke test passes.

13. Binary or source reduction must not be achieved by wholesale replacement of terminal rendering, terminal input, block rendering, terminal session/model, local PTY, or alternate-screen code with stubs.

14. If a removed product module owns types still needed by the terminal, those neutral types remain available through a local, service-independent boundary. The terminal must not depend on an AI or Warp-backend crate solely to reuse a data type.

15. Lite-only compile boundaries remain understandable to future upstream-sync work. A newly cherry-picked change cannot silently add a forbidden direct dependency to the lite application.

16. Upstream terminal, renderer, shell, security, and performance fixes remain eligible for vetted cherry-picks. Changes that require restored AI/cloud/account behavior remain excluded or are adapted behind the non-lite boundary.

17. Failures are conservative:
    - If compile-boundary work breaks the build, the affected phase is not advanced.
    - If the native application launches without a usable window or terminal, the change is treated as failed even if compilation succeeds.
    - If privacy verification detects an unexpected outbound connection, the release is blocked until explained and resolved.
