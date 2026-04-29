# Fork Notice — warp-lite

**warp-lite** is a derivative work of [warpdotdev/warp](https://github.com/warpdotdev/warp), the open-source Warp Terminal released by Denver Technologies, Inc. in 2026.

## Provenance

- **Upstream:** https://github.com/warpdotdev/warp
- **License (preserved):** AGPL-3.0-only (with the original `warpui` and `warpui_core` crates remaining MIT, as in upstream)
- **Original copyright:** Copyright (C) 2020–2026 Denver Technologies, Inc.
- **Fork copyright on modifications:** © 2026 warp-lite contributors. AGPL-3.0-only applies to all modifications.

This fork is published under AGPL-3.0-only as required by the upstream license. The fork **cannot** be relicensed (only Denver Technologies, the original copyright holder, may do that); modifications introduced by this fork are also AGPL-3.0-only.

## What was removed

This fork strips subsystems that the maintainers of warp-lite consider unnecessary for a privacy-respecting local terminal. Removed in early phases:

- **AI subsystem** — `crates/ai`, `crates/computer_use`, agent mode UI, MCP integration, ambient agents, AI completions
- **Warp Cloud** — `crates/firebase`, `crates/graphql`, `crates/warp_server_client`, `crates/websocket`, Warp Drive, session sharing, account login
- **Onboarding wizard** — `crates/onboarding`, first-launch agent setup
- **Telemetry & crash reporting** — Rudderstack call sites, Sentry crash uploads
- **Niche side features** — voice input, install detection HTTP route, sleep prevention guard, Warp's custom plugin host

Editor (Zed-fork), LSP, language tree-sitter integrations, Vim mode, and the terminal core (block model, GPU rendering, shell integration) are kept. See [the implementation plan](https://github.com/terzigolu/warp-lite/commits/warp-lite/main) for commit-by-commit removal history.

## Branch model

- `warp-lite/main` — the divergent line for this fork.
- `upstream-tracking` — periodic mirror of `warpdotdev/warp@master`. Direct merges from `upstream-tracking` into `warp-lite/main` are **not** performed; targeted fixes are integrated via `git cherry-pick -x <sha>` to preserve attribution.

## AGPL §13 disclosure

If this fork is offered over a network (e.g., remote pair-programming, hosted shell access), AGPL §13 obligates the operator to make the corresponding source code available to all interacting users. The canonical source is this repository.

## Trademarks

"Warp" is a trademark of Denver Technologies, Inc. This fork uses the Warp source code under AGPL but **does not** claim affiliation, endorsement, or sponsorship by Denver Technologies. The fork is named *warp-lite* to signal the derivative relationship while making the lightweight intent explicit.

If the upstream rights-holders ask for a name change, we will rename promptly.
