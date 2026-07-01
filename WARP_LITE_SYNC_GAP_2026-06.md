# warp-lite ↔ upstream/warp Sync Gap Analizi (2026-06-30)

- **Fork noktası:** `bc3fffa` (2026-04-28)
- **upstream/master:** `d375729` (2026-06-07) — fork'tan **940 commit** ileride
- **Bizim branch:** `warp-lite/sync-2026-06` — fork'tan 91 commit (76 lite-purge + **16 cherry-pick'lenmiş upstream PR**)
- Yöntem: 940 commit → 391 AI/cloud/agent elendi → 550 aday → tema bazlı küratörlük.

## ✅ ZATEN ALINMIŞ (sync-2026-06'da cherry-pick'li, tekrar alma)
#11673 zsh bracketed-paste · #11293 pane flex reset · #11197 right tab activate ·
#11099 split-pane footer clip · #10965 min window size · #10811 inline menu clip ·
#10581 vim zz · #10472/#10612 Clear context menu · #9444 $CDPATH completion ·
#11428 secret redaction deadlock · #11141 Mermaid WASM · #10824 unsaved changes UI ·
#10682 file watcher panic · #10332 async command palette · #10132 tmux statusline · #9665 core-text font leak

---

## 🟥 TIER 1 — Yüksek değer, lite ile birebir uyumlu (öncelik)

### Crash / Deadlock / Race (kararlılık)
| PR | Açıklama | Not |
|----|----------|-----|
| **#10308** | Fix deadlock in terminal view rendering | ⭐ CORE — terminal render kilidi |
| **#10241** | Fix race: requested commands auto-cancelled | ⭐ komut iptali yarış |
| #10666 | Fix run command executor race condition | komut yürütme |
| #10943 | Fix discard files panic | |
| #10265 | Fix race in git branch/diff-stats chip init | git chip |
| #9998 | Cloned repo stuck in loading (project explorer) | |

### Render / Font / Scroll
| PR | Açıklama | Not |
|----|----------|-----|
| **#11454** | Add Intel HD Graphics 2500 to buggy iGPU adapters | ⭐ GPU çökme guard |
| **#10445** | Fix soft-wrapped lines glyph indices (cosmic text) | ⭐ metin render |
| **#9624** | Scroll output Page Up/Down from prompt | ⭐ |
| #9452 | Fix git ops flicker | |
| #9448 | Text selection auto-scroll when dragging beyond bounds | |
| #9320 | Fix file tree loading flicker | |
| #9332 | scroll-to-selected-block keybinding when editor focused | |
| #10683 | Enable blocklist markdown table rendering by default | |

### Shell / Terminal Core
| PR | Açıklama | Not |
|----|----------|-----|
| **#9279** | Update tab CWD + git branch from OSC 7 escape sequences | ⭐ core OSC |
| **#11868** | Fix empty RPROMPT handling in zsh | ⭐ zsh |
| **#10615** | Fix bash HISTSIZE startup sentinel | ⭐ bash |
| #11789 | Fix working directories clean up | |
| #11203 | Tolerate localized PowerShell executable output | |
| #9499 | Consolidate powershell history loading errors | |
| #9345 | Recognize .command files as shell scripts | |
| #9503 | Run executable shell scripts in terminal (not editor) | |
| #10713 | Fix restored command history after SSH close | SSH ama lokal history mantığı |

### Input / Keybind / Perf hot-path
| PR | Açıklama | Not |
|----|----------|-----|
| **#10927** | Remove bootstrap block conditionals from input hot path | ⭐ input latency perf |
| **#9514** | Fix meta+enter/tab/escape sending literal key names (legacy encoding) | ⭐ |
| #10582 | vim ctrl-d/ctrl-u half-page scroll (zz alındı, bu alınmadı) | ⭐ eksik kalan ikiz |
| #9491 | Copy keybinding prioritize input text over selected blocks | |
| #9476 | Fix chord shortcuts on Windows non-Latin layouts | Windows |
| #9555 | active_cursor_position for IME positioning | IME |
| #9730 | Don't submit form when Enter confirms Japanese IME | IME (macOS) |

### Window / Tab / Pane bugfix
| PR | Açıklama | Not |
|----|----------|-----|
| **#9536** | Fix session restoration for maximized/fullscreen windows | ⭐ |
| #10855/#10083 | Fix 1px window restore after macOS green-tile | macOS |
| #11418 | Multiple connection across tabs → 'unable to load content' | |
| #9474 | Terminal background darkening in horizontal tabs mode | |
| #9283 | Vertical tabs panel not opening when tab bar = Always | |
| #9297 | Clip warping-indicator chips overflow narrow panes | |
| #10654 | Environment picker empty state full-width fix | |
| #10178 | Reserve titlebar space for window controls | macOS |

---

## 🟩 TIER 2 — Yeni özellikler (opt-in, terminal UX — istersen al)

### ⭐ TAB GROUPS (büyük yeni özellik, çok-commit'li seri)
Dikey + yatay sekme gruplama. Saf terminal UX, AI/cloud bağı yok.
`#11486` (flag+entry) → `#11749` (basic vertical) → `#11791` (actions) → `#11842` (rename) →
`#11903` (more options menu) → `#12000` (drag) → `#12089` (horizontal tab group render) ·
ek: `#11294` (vertical tab ownership) · `#11611` (notification dots) · `#11138/#11412` (focused picker Space)

### ⭐ CROSS-WINDOW TAB DRAGGING
`#9275` (support) · `#9991` (internal users) · `#12046` (preview enable)
> Not: bunlar feature-flag arkasında — promote etmek gerekebilir.

### Diğer küçük özellikler
| PR | Açıklama |
|----|----------|
| #9658 | CycleMostRecentTab — 3. Ctrl+Tab davranışı |
| #9351/#9712 | Rename Active Pane keyboard-bindable action |
| #9305 | /set-tab-color slash command |
| #10334 | "Reveal in Finder" code pane overflow menu |
| #10120 | Tab context metadata copy actions |
| #10012 | Configurable code editor line numbers |
| #9347 | Reopen Closed Session (Linux/Windows menüsü) |
| #10293 | Rich text editor newlines as line breaks |
| #9655 | `warposs://pane/{uuid}` deep link (zaten oss şemamızla uyumlu) |

---

## 🟨 TIER 3 — Platform/altyapı (Windows/SSH build'i hedefliyorsan)
Windows: #10976 (conpty güncel) · #10442 (non-IME Unikey/EVKey) · #9891 (quake mode) · #9987 (reset grid restored blocks) · #9863 (installer) · #9528(?) · #10528 (zombie minidump auto-update) · #9498 (autoupdate error detect)
WSL: #10137 (Linux-side git/gh) · #9322 (/open-file path joining)
Refactor (opsiyonel, upstream'le hizalı kalmak için): #11672 (Metal renderer→objc2) · #11667 (keycode cache→objc2) · #11387 (warp_assets crate)

---

## ⛔ ELENENLER (lite ile çelişir → ALMA)
- **command-signatures bump'ları** (#11764) — `command-signatures-v2` crate'ini sildik.
- Agent/handoff/queued-prompt/rich-input-as-agent: #11575, #11848, #11723, #10389, #8a4df58a, #10859, #11083
- Remote editor/buffer sync (cloud collab): #10520, #10772, #10819, #10703, #11691
- Shared session / SSE / context window / V4A diff: #10780, #11574, #1b6642f2, #10186, #89f61b63
- Microphone (#12074), changelog/CI/stakeholders/owners, Slack handoff
```
```
