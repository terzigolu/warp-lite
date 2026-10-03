# Warp Lite October sync follow-up

This follow-up starts from released `v0.5.8-lite` (`4286947a2344df96dccf529e0c718431b6f90709`) and re-examines the same 368 upstream commits through `c157d411d988176506a4a7278890f626a22413dd`. The user explicitly expanded scope to include the remaining compatible improvements. The earlier byte-for-byte `warpui` / `warpui_core` boundary no longer applies to this work.

## Included

Changes from 14 additional upstream sources were adapted. This is a selective adaptation, not 14 whole cherry-picks. The original v0.5.8 source report remains a historical snapshot; [follow-up inventory](docs/upstream-sync/2026-10-03-followup.json) records the new dispositions and source SHAs.

- Opt-in right-click paste with Shift+right-click context menu, preserving mouse-owning terminal programs. Applies to blocks, alternate screen, input, rich input, and prompt chips. Existing scroll callback API stays intact.
- macOS `public.tiff` clipboard image detection, including the existing CLI-agent paste path.
- Uncached layout for changed editor text, avoiding the discarded layout cache and retaining font fallback requests. Placeholder text still uses the persistent layout cache. Includes Unicode/BOM, shaping, caret-offset tests and an adapted benchmark.
- RTL smart-selection panic guard and font metric fallback when a glyph has advance but no outline bounds.
- Smaller UI update/spawn monomorphization, adapted to Lite's existing view storage. No local performance percentage is claimed.
- Windows WGPU device-loss recovery, backdrop materials (None, Mica, Acrylic, Mica Alt), and migration of the legacy Acrylic preference using local settings initialization.
- cosmic-text pin that excludes Hack as a fallback donor on non-CoreText platforms.
- Grok Build local CLI identity, rich input, OSC 9/777 notifications and explicit user-triggered hook installation. Duplicate fallback notifications are suppressed after rich events; another agent's identity is rejected.
- Kiro local CLI identity, `kiro-cli` / `kiro` detection, icon and rich input. This does not add Kiro authentication or a notification plugin.
- Once-per-run flex diagnostics and full error-chain formatting for local keybinding errors, without restoring Sentry reporting.

The local `pluggable_notifications` feature is now enabled in the default build so OSC 9/777 CLI events actually reach the existing session handlers. Embedded AI/cloud/telemetry remain disabled, and tmux layout is retained. Added agent enum values support the existing local CLI integrations; they do not enable embedded Warp Agent or cloud inference.

## Remaining exclusions

The follow-up inventory replaces the obsolete protected-UI explanation for all previously held entries. Agent TUI runtime changes have no corresponding runtime in Lite. Thought selection, cloud connection status, team switching and usage billing belong to disabled product surfaces. Secret-detector extraction and headless renaming are structural changes with no additional local-terminal behavior to import. The rmcp 0.10-fork to 2.2 migration remains separate; this follow-up does not claim advisory remediation or non-applicability.

## Validation

Implementation commits: `15a830397ae56c572e6c0c27a33dcd5d83b28921` (upstream adaptations) and `a86f91850` (native-validation corrections and regression tests). The final build uses the latter source revision.

- 1,258 selected tests passed and 7 were ignored. UI core/editor: 744 passes, 7 ignored; uncached/cached Unicode and BOM layout parity: 1 pass; application: 512 passes; native macOS TIFF-only pasteboard: 1 pass.
- One input test fails because it expects the disabled AI context menu. It also fails in isolation; no full baseline application build was used to establish historical test status. Whole-suite success is not claimed.
- The font and RTL regression tests fail in four cases with only their production files restored to v0.5.8, then pass with the fixes restored.
- Command detection no longer assumes a Grok/Codex notification plugin is installed; only the explicit install flow seeds the minimum version. Regression tests cover both paths.
- Three existing terminal tests leaked global AgentView/CloudMode flags. Switching them to scoped overrides makes all 96 terminal view tests pass together, including six newly imported right-click tests.
- Final default Cargo check and the new benchmark's compilation passed. The `warp_platform` check passed at the first implementation revision, before the final notification correction.
- Clippy passed with `-A clippy::overly_complex_bool_expr` for the existing disabled AI-panel gate (`false && ...`). Without that exception, the gate triggers a deny-by-default lint. Other warnings remain; this is not a `-D warnings` pass.

- Final optimized macOS release build passed and the separate test app passed strict ad-hoc signature verification. Native launch, shell commands and Turkish/wide-character output passed.
- A real TIFF-only macOS clipboard paste into a synthetic Grok OSC 777 session delivered the expected Ctrl+V (`0x16`) to its PTY. This does not validate authenticated Grok/Kiro provider requests. The user clipboard was restored. The existing Applications installation was not replaced.
- Candidate bundle: `/tmp/warp-lite-followup-20261003/WarpLite Followup.app`. No new release was published.

The [validation record](docs/upstream-sync/2026-10-03-validation.json) separates native checks from automated tests. Windows/Linux runtime behavior and authenticated Grok/Kiro requests require their respective environments.
