# Warp Lite upstream sync — 2026-08-15

## Sonuç

Warp Lite'ın son senkron tabanı olan `6658d353848a8f458b5b87ce30297bf686b1f47f`
(`warp-lite/main`) ile incelenen upstream snapshot
`a9c0a1ebda0acfe5e57b6f6df7c6ef744a71f8eb` (`upstream/master`) arasındaki
1.143 commit tarandı.

`warp-lite/sync-2026-08` dalına, Warp Lite sınırına uyan güvenlik, gizlilik,
terminal güvenilirliği, editör/dosya deneyimi, performans, macOS/Windows/WSL/SSH
uyumluluğu ve geliştirici araçları değişiklikleri alındı. Nihai dal 195 commit
içerir: 193 senkron uygulama commit'i ile bir tasarım/plan ve bu rapor. Rapor
dahil nihai diff 286 dosyadır; kod diff'i rapor öncesinde 285 dosya,
20.633 ekleme ve 2.530 silmeydi.

Uygun değişiklikler doğrudan cherry-pick edildi; upstream'in silinmiş veya
Warp platformuna bağlı API'lerine dayanan yararlı parçalar ise küçük, yerel
Warp Lite uyarlamaları olarak taşındı. Uygulanan commit manifestinin kesin
kaynağı şudur:

```bash
git log --reverse --oneline warp-lite/main..warp-lite/sync-2026-08
```

## Alınan başlıca değişiklikler

### Güvenlik ve gizlilik

- Markdown link açma, iTerm dosya indirme, display-chip RCE, code-search komut
  enjeksiyonu, SSH komut enjeksiyonu ve repo dizini shell escaping düzeltmeleri
  (`1f7460146`, `6ad5d2c8c`, `6cd0d52f0`, `c3fa1004e`, `f0ddba1d3`,
  `14aa3164b`, `7f85ae753`).
- Komut blocklist kontrolünden önce environment assignment ayıklama
  (`c9e9f01a5`), auth URL log redaksiyonu (`21c810b96`) ve MCP secret-redaction
  tercihini koruma (`1a9d95801`).
- OSC 52 clipboard okumasını kullanıcı ayarıyla kapatma/açma ve engellendi
  bildirimi (`07db02cff`, `ed60747d3`).
- H2 debug panic düzeltmesi ve lockfile uyarlaması (`758591ca6`, `1f2f6f96b`),
  `serde_with` güvenlik güncellemesi (`cca0b9248`).

### Terminal, shell ve uzak oturum güvenilirliği

- Bloke PTY write sırasında response sequence kaybı, zsh grid bozulması,
  wrapped-line path algılama, multiline command prefix tekrarı, wide-character
  resize crash ve startup inline image düzeltmeleri.
- OSC 8 hyperlink desteği varsayılan açıldı; OSC 1337 eksik parametre panic'i
  ve boş OSC hyperlink edge-case'i düzeltildi.
- Shell PATH yakalama, quoted home/worktree path, PowerShell history/bootstrap,
  zsh explicit-width prompt ve SSH wrapper/RemoteCommand davranışları
  sertleştirildi.
- Generator komutları ayrı process group'ta çalışıyor ve cancellation sırasında
  alt süreçleriyle birlikte öldürülüyor (`a5365a37a`, `955171a88`). Büyük PTY
  environment'ları artık `E2BIG` öncesinde fail-fast davranıyor (`9483c9c4f`).
- Remote writer kapanış hatası crash-report seviyesinden normal log seviyesine
  indirildi; shell exit reason teşhis loguna eklendi.

### Editör, dosyalar ve Vim

- Non-ASCII find/replace, Markdown syntax highlight, HTML comment gizleme,
  header/table selection, yerel Markdown image refresh ve viewer preference
  düzeltmeleri.
- File viewer'a `Copy file path`, path ve URI kopyalama; file-only aramada
  dizinleri dışlama; file tree'de doğal sayısal sıralama.
- Text editor autosave ayarı Warp Lite settings/event API'sine uyarlandı.
- Vim için count+`gg`, visual paste, `d%`/`c%`/`y%`, indent/dedent, environment
  variable editor Vim modu, find refocus ve soft-wrapped visual-line hareketleri.

### Sekmeler, gruplar ve masaüstü UX

- Horizontal/vertical tab grouping ve pinning zinciri; persistence,
  cross-window drag, pin/group invariant'ları, renkler, rename/collapse,
  multi-pane header ve crash düzeltmeleri.
- Fullscreen corner, title bar search gizleme, tab background/contrast,
  active-tab color cycling, küçük pencerede suggestion bounds ve session menu
  max-height düzeltmeleri.
- Quake window focus, dedicated hotkey window Dock görünürlüğü ve macOS native
  window chrome/zoom davranışı.
- Warp'ın yaygın dosya tiplerini otomatik sahiplenmesi durduruldu
  (`LSHandlerRank=Alternate`).

### Performans, platform ve geliştirici deneyimi

- Gitignore kurallarını event'ler arasında paylaşma; filesystem-only watcher'da
  Git işini atlama; dizin symlink target'larını izlememe; gereksiz path
  canonicalization ve ignored-directory rebuild azaltmaları.
- Core Text öncesi aynı style-run birleştirme ve box-drawing glyph'lerini
  procedural render etme.
- Windows'ta her session için full process-table CPU taramasını kaldırma;
  WSL UNC hostlarını case-insensitive tanıma; eski Mesa kullanan Intel Xe
  adaptör allowlist'i.
- macOS bootstrap'ta headless/non-interactive Homebrew, transient retry,
  codesign timestamp retry, doğrulanmış `cargo-binstall` ve kurulan tool binary
  doğrulaması.
- `app/build.rs` path API düzeltmesi ve target directory'nin script tarafından
  doğru çözülmesi.

## Warp Lite sınırı ve dışarıda bırakılanlar

Toplu audit'te AI/agent davranışı, cloud hesap/obje akışları, billing, team,
orchestration, remote-control ve yeni telemetry toplama/gönderme değişiklikleri
alınmadı. Temsilî dışarıda bırakılan upstream commit'ler:

- `e367c9de`: queued prompt / AI davranışı.
- `912e4540`: AI Markdown akışı.
- `9d3f3e1e`: cloud/AI bağımlı değişiklik.
- `63b582890`: agent SDK.
- `a1af68cbd`: Lite'ta bulunmayan MCP JSON viewer bileşeni.
- `43c21508`: Lite'ta olmayan lifecycle mimarisi.
- `d4c4cf9b`: Lite'ta bulunmayan `warpctrl` altyapısı.
- `af29c593b`: Lite'ta bulunmayan action'a bağlı Copy Current Path.
- `89f742fa`: managed-secrets/platform sınırı.
- `d56d70ade`: Lite'ta silinmiş şemaya bağlı değişiklik.

Default-feature diff yalnızca şunları ekler:

```text
osc_hyperlinks
grouped_tabs
pinned_tabs
```

Diff'te yeni `send_telemetry`, `send_event`, `report_event`, `track_event`,
`emit_telemetry` veya `log_telemetry` çağrısı yoktur. AI dizinlerinde değişen
dosyalar yalnızca mevcut/dormant kaynakta güvenlik sertleştirmeleri ile yerel
Markdown image refresh desteğidir; AI veya agent feature'ı açılmadı.

## Doğrulama

| Kontrol | Sonuç |
| --- | --- |
| `cargo check -p warp --bin warp-oss` | Başarılı |
| `cargo check -p warp --bin warp-oss --features warp_platform` | Başarılı |
| `cargo test -p repo_metadata` | 48 geçti, 0 kaldı, 3 ignored |
| `cargo test -p warp_util` | 64 unit + 3 doctest geçti |
| WSL UNC odaklı testler | 2/2 geçti |
| `cargo test -p warp_terminal` | 111 geçti, 0 kaldı, 2 ignored |
| Terminal escape odaklı testler | 22/22 geçti |
| `cargo test -p vim` | 68/68 geçti |
| `cargo test -p markdown_parser` | 151/151 geçti |
| `cargo test -p warpui_core` | 288 geçti, 0 kaldı, 7 ignored; doctest 2 geçti, 1 ignored |
| Warp lib test binary, `--features agent_management_view` | Derleme başarılı |
| Local command executor process-group testleri, `agent_management_view,local_tty` | 3/3 geçti |
| `zsh -n` / `bash -n` bootstrap script kontrolleri | Başarılı |
| `git diff --check` | Başarılı |
| Conflict-marker taraması | Temiz |

Derlemelerde çok sayıda mevcut unused/dead-code ve unexpected-`cfg` uyarısı
vardır; bunlar error değildir.

## Bilinen doğrulama sınırları

- Varsayılan `cargo test -p warp --lib --no-run`, `Workspace` içindeki
  `agent_management_view` alanına iki unguarded test referansı nedeniyle
  derlenmiyor (`app/src/workspace/view_test.rs` civarı). Aynı test binary'si
  `--features agent_management_view` ile başarıyla derlendi; bu cfg uyumsuzluğu
  sync öncesinden kalan test-fixture borcudur.
- `cargo fmt --all`, repository'de referans verilen fakat bulunmayan iki eski
  disabled test modülü nedeniyle başlayamıyor:
  `app/src/search/ai_context_menu/_disabled_subtree/blocks.rs` ve
  `crates/ai/src/agent/action_result/convert_tests.rs`. Değişiklikler için
  `git diff --check` temizdir.
- Intel Xe/Mesa allowlist testi macOS'ta platform `cfg` nedeniyle derlenmiyor;
  ilgili kaynak check'ten geçmiştir ancak gerçek Linux/Intel runtime testi
  yapılmadı.
- Kurulu `/Applications/WarpLite.app` ve çalışan Warp Lite süreci korunmuştur;
  installed-app smoke testi yapılmadı.
- Herhangi bir push, tag, release, notarization veya publish işlemi yapılmadı.

## Handoff

İnceleme/merge için dal: `warp-lite/sync-2026-08`.

Önerilen sonraki adım, bu dalı ayrı bir uygulama bundle'ı olarak build edip
mevcut kurulu uygulamaya dokunmadan manuel smoke test yapmaktır. Bu rapor build
ve kaynak doğrulamasını belgeler; release onayı değildir.
