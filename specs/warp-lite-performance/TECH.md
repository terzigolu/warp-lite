# Warp Lite Performance and Source Slimming — Technical Plan

## Context

The product contract is defined in [PRODUCT.md](./PRODUCT.md).

The current `warp-lite/main` baseline is `cb7f0823`. The release configuration already uses `debug = false` and `strip = "symbols"` in `Cargo.toml`, and the existing `target/release/warp-oss` is approximately 256 MB. The workspace currently contains 53 package manifests including `app`.

The remaining weight is not merely archival source:

- `app/Cargo.toml` directly depends on `ai`, `computer_use`, `onboarding`, `warp_graphql`, `warp_server_client`, and `websocket`.
- `app/src/lib.rs` unconditionally declares product modules including `ai`, `auth`, `billing`, `cloud_object`, `code_review`, `drive`, `pricing`, `server`, `voice`, and `voltron`.
- `app/src/ai` is approximately 177k Rust LOC.
- `app/src/server` is approximately 38k Rust LOC.
- `app/src/drive` and `app/src/code_review` are each above 20k Rust LOC.
- `cargo tree -p warp-oss -i <crate>` confirms that the release application remains a consumer of the AI and backend client crates.

The protected terminal core includes:

- `app/src/terminal/view.rs`
- `app/src/terminal/input.rs`
- `app/src/terminal/block_list_element.rs`
- `app/src/terminal/view/`
- `app/src/terminal/input/`
- `app/src/terminal/local_tty/terminal_manager.rs`
- `app/src/terminal/alt_screen/alt_screen_element.rs`
- `app/src/terminal/model/{block,blocks,terminal_model,session}.rs`

These files may receive narrow import/type-boundary edits, but they must never be wholesale stubbed or bulk deleted.

## Implementation status

- Phase 0 static benchmark tooling is implemented in
  `script/benchmark-warp-lite.sh`.
- Phase 1 runtime harness is partially implemented in
  `script/benchmark-warp-lite-runtime.sh`: process/window readiness proxies and
  idle RSS/CPU sampling exist. During active development it is validated only
  with `--dry-run`; a real cold/warm baseline requires an isolated window with
  no existing Warp Lite process. First shell-prompt readiness and sustained
  output/large-scrollback measurements remain Phase 1 follow-ups.
- The Phase 2 boundary `warp_platform = []` exists in `app/Cargo.toml`. The
  first compile-green leaf slice places the 475-LOC `billing` module and its
  shared-object-capacity modal wiring behind that boundary. The default Lite
  build ignores the corresponding Drive panel event and omits the capacity
  gate; `--features warp_platform` preserves the existing behavior.
- The next leaf slice removes the 1,131-LOC Referrals settings page and the
  Account-page referral CTA from the default Lite compilation. It also avoids
  constructing that settings page's referrals backend client. The section and
  navigation vocabulary remain intact for upstream compatibility, while
  `--features warp_platform` restores the page and CTA.
- The reward slice removes the 214-LOC referral reward modal module and all
  Workspace modal construction/render/event wiring from the default Lite
  compilation. The startup subscription and `query_referral_status` call are
  also platform-only, so Lite no longer constructs a referrals client or
  initiates that reward-status backend request during Workspace startup. The
  query methods and their auth/server/referral imports are gated at the model
  boundary as well, rather than remaining as unreachable default-build code.
  Local referral-theme preference state remains available to the theme chooser
  until that ownership boundary is handled separately.
- The remaining referral entry points are platform-only: the Resource Center
  invite button, user-menu invite item, command-palette binding, Workspace
  action variant, and action handler are absent from the default Lite build.
  This closes the referral UI cluster without introducing a placeholder page
  or no-op action.
- The first pricing-dependent terminal leaf places the 857-LOC
  `terminal/buy_credits_banner` module behind `warp_platform`. Default Lite no
  longer constructs its dropdown, pricing/AI/workspace subscriptions, purchase
  state, overlay checks, or keymap context for every terminal Input. The
  protected Input renderer files receive only narrow conditional
  import/field/construction/render-call edits; their terminal behavior is not
  stubbed or rewritten.
- The remaining low-risk pricing UI leaves are platform-only: the free-tier
  limit modal, 869-LOC build-plan migration modal, 483-LOC auto-reload modal,
  and 3,686-LOC Billing & Usage settings page plus its usage-history support
  module. Their Workspace fields, subscriptions, render paths, settings page
  handles, and command entry points are omitted in Lite. A duplicate
  auto-reload modal construction/subscription in the platform path was removed
  while consolidating construction through the existing builder.
- Pricing updates from workspace metadata, the agent-onboarding price badge,
  and the AI usage model's auto-reload price calculation are platform-only.
  `PricingInfoModel` and the complete pricing module are now platform-only.
  The retained Teams settings page computes exact per-seat prices on platform
  builds and uses a generic per-user billing description in Lite, avoiding a
  pricing-service singleton solely for explanatory UI copy.
- AI, legacy AI-assistant panel, agent editor/status-bar, and AI todo popup
  initialization/keybinding registrations are platform-only. This is the first
  AI startup slice. The scheduled ambient-agent manager, its schedule command
  implementation, and the Agent status-bar tip singleton are also platform-only;
  Lite returns the existing `invalid value 'schedule'` result at the CLI
  boundary instead of exposing a route to an absent singleton.
- A default Lite release build after these slices produced a
  267,208,032-byte `warp-oss`, down from the same-machine baseline of
  268,410,752 bytes: 1,202,720 bytes (approximately 1.15 MiB) smaller. The
  installed application bundle was intentionally not rebuilt or replaced while
  it was hosting the active development session.
- Resolved-graph checks still show `ai`, `computer_use`, `onboarding`,
  `warp_graphql`, `warp_server_client`, `websocket`, and
  `warp_managed_secrets`. Their remaining consumers cross protected
  terminal/workspace state and persistence contracts, so dependency
  optionalization is not a safe mechanical follow-up to this slice.
- Both `cargo check -p warp` and
  `cargo check -p warp --features warp_platform` compile successfully. This
  work removes source from the default compilation unit but does not yet remove
  a direct dependency or establish a binary/runtime improvement.

## Proposed changes

### Phase 0 — Measurement contract

Add `script/benchmark-warp-lite.sh` as the single static-baseline command.

The script:

- Runs without modifying the repository unless the caller explicitly requests
  `--output`. Existing output files are refused unless `--force-output` is
  supplied.
- Uses tools already required by the repository (`bash`, `git`, `find`, `wc`, `stat`, `du`, `cargo`).
- Prints a stable `key=value` report suitable for diffing or CI parsing.
- Accepts `--output <path>` to persist the same report.
- Reports missing build artifacts instead of triggering a build.
- Reports whether forbidden direct dependencies remain in the current inline
  `app/Cargo.toml` syntax. This is an inventory metric, not the future release
  gate; Phase 6 uses Cargo metadata/the resolved graph so aliases, dependency
  tables, and target-specific declarations cannot bypass it.
- Reports targeted source LOC so a cleanup cannot be presented as complete while major product surfaces remain.

This phase establishes Behavior 6–8. It intentionally makes no performance claim.

### Phase 1 — Native runtime baseline

Add a macOS-only runtime harness after the static report is established.

The harness will:

1. Require an explicit `.app` path; defaulting to `/Applications/WarpLite.app` is acceptable only when present.
2. Canonicalize the selected executable and refuse before launch when any
   process already uses that exact executable path. There is no override during
   an active development session.
3. Terminate only the exact PID started by the harness, and only while its
   executable still matches. Never use `killall` or `pkill`.
4. Measure cold and warm launch using an application signpost or a narrowly scoped readiness marker. Wall-clock delay alone is not sufficient.
5. Sample resident memory and CPU after a fixed 30-second idle settling window.
6. Record machine model, macOS version, architecture, and build revision separately from comparable metrics.
7. Leave raw samples under `target/warp-lite-benchmarks/`, which remains untracked.

Add a native smoke checklist for window visibility, shell prompt readiness, command execution, pane split, tab creation, alternate-screen entry/exit, and large output. This validates Behavior 1–3 and 9–13.

### Phase 2 — Introduce the compile boundary

Do not begin by deleting modules. First make product ownership explicit.

1. Inventory `app/Cargo.toml` features and map each direct AI/cloud/account dependency to its compile consumers.
2. Introduce one positive product boundary, tentatively named `warp_platform`, for upstream Warp service features. Warp Lite defaults must omit it.
3. Gate top-level product modules in `app/src/lib.rs` and their initialization sites behind that boundary.
4. Keep neutral local types outside the gated modules. Where a terminal-core consumer imports a type from an AI/cloud crate, move the minimal type into an existing neutral crate when ownership is clear; otherwise introduce a small compatibility module inside `app`.
5. Add compile-time checks that the default lite feature set does not enable forbidden product features.
6. Use `cargo check -p warp --no-default-features --features <lite-feature-set>` after every small boundary edit.

Tradeoff: many fine-grained features provide precision but create an upstream-sync maintenance burden. One product boundary is preferred initially; split it only when a retained local feature genuinely needs part of the product graph.

### Phase 3 — Backend and account cut

Start with the lower-risk product cluster before the 177k-LOC AI UI:

1. `billing`, `pricing`, `referral_theme_status`, and `reward_view`.
2. Login/account UI while preserving the minimal local startup identity contract.
3. `warp_server_client` direct app use.
4. `warp_graphql` and managed-secrets consumers.
5. Cloud-drive/cloud-object paths.
6. Websocket/shared-session paths that are not required for local terminal operation.

After each slice:

- Run compile checks.
- Run relevant unit tests.
- Build the native release.
- Run the smoke checklist.
- Compare the Phase 0/1 baseline.

Only after reverse dependency count reaches zero should the corresponding crate or module be physically removed.

### Phase 4 — AI product cut

Treat `app/src/ai` as several ownership clusters, not one deletion:

1. Agent management and provider/account models.
2. AI document and conversation surfaces.
3. AI context menu and prompt prediction.
4. Codebase indexing and embeddings.
5. AI assistant panel state and compatibility persistence.
6. Remaining neutral command/editor types.

Each cluster gets its own compile-green and native-smoke checkpoint. `crates/ai` is removed only when `cargo tree -p warp-oss -i ai` has no normal dependency path.

### Phase 5 — Runtime profiling and targeted optimization

Profile the native app after product tasks are excluded so measurements represent the intended architecture.

Candidate areas are hypotheses, not automatic deletion targets:

- One-second terminal attribute polling.
- PTY throughput metric timers.
- Shared-session heartbeats that should not exist in a local-only build.
- Language-server lifecycle scans.
- Workspace render invalidation and repeated `notify` chains.
- Filesystem/process queries from render paths.
- Image/SVG rasterization and cache behavior.
- Large-scrollback allocation and cloning.

Use Instruments Time Profiler, Allocations, and signposts for macOS. Optimize only a measured hot path, then rerun the same scenario.

### Phase 6 — Size and regression gates

Add CI/local presubmit checks for:

- Forbidden direct dependencies in the lite build.
- Release binary and bundle size budgets.
- Static benchmark report generation.
- Zero telemetry call sites.
- Privacy smoke verification where the environment supports it.

Do not choose final budgets until Phase 1 records repeatable measurements. Initial warning thresholds should use the recorded baseline plus a small regression tolerance, not an aspirational arbitrary number.

### Phase 7 — Upstream-sync contract

Extend the dated upstream-sync audit with:

- Default-lite feature compilation.
- Forbidden dependency diff.
- Static size comparison.
- Native launch smoke.
- Runtime benchmark comparison for large renderer/workspace changes.

Keep `git cherry-pick -x` provenance and the existing allowlist approach.

## Testing and validation

### Phase 0

- Run `script/benchmark-warp-lite.sh`.
- Run it again with `--output target/warp-lite-benchmarks/baseline.txt`.
- Diff stdout and the persisted report, excluding no fields; they must match.
- Test the script against missing binary/app paths through `--binary` and
  `--app`.
- Verify an existing `--output` path is refused unless `--force-output` is
  supplied.

### Compile-boundary phases

- `cargo check -p warp`
- `cargo check --workspace` at phase boundaries.
- Reverse dependency checks:
  - `cargo tree -p warp-oss -i ai`
  - `cargo tree -p warp-oss -i warp_graphql`
  - `cargo tree -p warp-oss -i warp_server_client`
  - `cargo tree -p warp-oss -i websocket`
- Tests nearest each edited module.

### Native validation

Map to PRODUCT Behavior 1–4 and 9–13:

- Build release binary and `WarpLite.app`.
- Install the release-built app into `/Applications`.
- Verify a visible usable window and ready shell prompt.
- Execute a command, create a tab, split a pane, search history, enter/exit an alternate-screen application, and produce large output.
- Verify idle outbound connections with an explicit observation window.
- Capture cold/warm launch, idle RSS/CPU, and sustained-output measurements.

### Regression acceptance

- No protected terminal-core file is wholesale stubbed.
- No new login, telemetry, AI, cloud, billing, or account UI appears.
- No unexpected outbound connection appears.
- Every claimed improvement is backed by before/after output from the same revision-aware harness.

## Risks and mitigations

- **Compile success but broken GUI:** native smoke is mandatory at every product-cluster boundary.
- **Shared types keep heavy crates alive:** move only neutral types, one ownership boundary at a time; avoid copying complete service APIs into compatibility stubs.
- **Feature-flag explosion:** begin with one `warp_platform` boundary and split only with evidence.
- **Misleading benchmarks:** pin scenario, settling time, build profile, and machine metadata.
- **Upstream sync conflict growth:** keep gates close to module declarations and Cargo dependencies, and document every rejected upstream product dependency.
- **Over-optimizing source LOC:** prioritize binary graph and measured runtime metrics over raw deletion counts.

## Implementation sequence

1. Land Phase 0 benchmark tooling and baseline.
2. Land Phase 1 native measurement harness.
3. Draft the exact `warp_platform` dependency matrix.
4. Cut backend/account cluster in small compile-green slices.
5. Cut AI clusters.
6. Profile and optimize the resulting lite runtime.
7. Add regression gates and update the upstream-sync runbook.
