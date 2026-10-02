# Warp Lite upstream sync — 2026-10-02

## Yerel kurulum ve native smoke testi — takip doğrulaması

Kullanıcının sonraki build/Applications kurulum talebi üzerine `dff37b11025798b03f09dea6d6d9a3819144d4a2` kaynak noktasından `cargo build --release --bin warp-oss` başarılı oldu (son build komutu 5m45s). Paket `script/build-warp-lite-app.sh` ile üretildi ve `/Applications/WarpLite.app` içine kuruldu. Bu takip doğrulaması aşağıdaki ilk incelemede belirtilen yerel release build/kurulum/native smoke eksikliğini kapatır; diğer platform ve tüm özellikler için uçtan uca doğrulama değildir.

- Eski paket: `/Applications/WarpLite.app.backup-before-sync-20261002-215335`.
- Kurulu arm64 Mach-O UUID: `DB110357-F5F1-3972-8104-4FCE0671E575`.
- İmzalı executable SHA256: `8688897747eb5e4b6979e57d44e467fc8e51d38b4518191e40fb773ba580d505`.
- Ad-hoc imza strict doğrulaması, kaynak paket/kurulu executable hash eşitliği ve UUID karşılaştırması geçti. Notarization yapılmadı.
- Sürüm etiketi `0.5.7-lite` olarak kaldı; `Info.plist` içindeki `WarpLiteSourceRevision`, `WarpLiteSourceBranch` ve `WarpLiteLocalBuildDate` yeni yerel build'i ayırt eder.
- Native uygulama açıldı; zsh hazır geldi. `printf`, Türkçe çıktı, `U+754C` geniş karakter, Tab ile `installed-b` → `installed-build.json` tamamlaması, yeni sekme ve ikinci sekmede komut yürütme gözlendi.
- Accessibility çıktısı ve gerçek pencere screenshot'ı kontrol edildi. Çalışan ana/yardımcı süreçlerin `/Applications/WarpLite.app/Contents/MacOS/warp-oss` yolundan geldiği doğrulandı (kontrol anında PID 58661 / 58669). Uygulama kullanıcı için açık bırakıldı.
- Kurulum/smoke kanıtı: `/tmp/warp-lite-sync-20261002/installed-build.json`; build ve kurulum logları aynı geçici dizinde. Push veya public release yapılmadı.

## Sonuç ve kapsam

Çalışma dalı: `warp-lite/sync-2026-10`. Kod son noktası: `b3b3adbeeb42070fd9bb9b481eb161d0743cbd94`.

- Yerel başlangıç / `origin/warp-lite/main`: `878dd3ed3e140ac26e2a95f1120365f5383d7775`.
- Önceki upstream inceleme noktası: `a9c0a1ebda0acfe5e57b6f6df7c6ef744a71f8eb` (2026-08-15).
- Bu incelemenin upstream noktası: `c157d411d988176506a4a7278890f626a22413dd` (2026-10-02).
- Aralıktaki **368 commit** değerlendirildi. **73 committen uyumlu değişiklikler** alındı veya Lite'a uyarlandı. Bu sayı, son imza paketi pininde birleştirilen altı ara pin güncellemesini de içerir; 73 bağımsız tam cherry-pick anlamına gelmez.
- Ek olarak aralıktan önceki `dc21551936ce85b0cb3945cb83705a3aadd60b38` POSIX/PowerShell satır sonu düzeltmesi alındı. Yeni regresyon testleri bu eksik ön koşulu ortaya çıkardı.
- Seçici backport yapıldı; upstream ile merge yapılmadı. Dolayısıyla Git ancestry üzerinden “upstream ile sıfır commit geride” sonucu beklenmez.

Tam envanter: [2026-10-02-commits.json](docs/upstream-sync/2026-10-02-commits.json). Her upstream commit için konu, dosyalar, karar ve gerekçe; alınanlar için yerel commit eşlemesi içerir.

## Dahil edilen davranışlar

- Bash, zsh, fish ve PowerShell için native tamamlama; boş sonuçlarda dosya yolu fallback'i; iptal ve eski sonuç kontrolleri; POSIX `--` argümanları ve güncel komut imzaları.
- fzf/atuin geçmiş ve dosya seçicilerine devir; fzf.fish ve Alt-C; zsh sorgu aktarımı. Seçim yalnızca etkin shell oturumundan ve bekleyen devirle eşleşiyorsa kullanılır.
- Bash PS1 çift genişletme, generator iptali, PowerShell vi-mode ve 5.1 giriş komutları; shell'e uygun satır sonları ve PowerShell kill-buffer yazımı.
- WSL symlink tamamlaması ve dizin göstergelerinde gereksiz guest süreçlerinin azaltılması.
- Yerel komut geçmişinde eşleşme kalitesi, yakınlık, oturum ve çıkış durumu sıralaması; arama indeksinin art arda yeniden oluşturulmasının birleştirilmesi.
- Paylaşılan ve sınırlandırılmış gitignore önbelleği; sınırlı completion imza önbelleği; tembel tamamlanmış-blok verisi; editör kopyaları ve paralel iş yükünün azaltılması.
- Sekme/grup yeniden adlandırmada ve menü kapatmada odak; dikey gruplarda arama; modifier tutulunca kısayol ipuçları; launch configuration içinde grupların saklanması; undo-close koruması; düzenlenebilir New Window kısayolu.
- Geniş karakterlerin satır sınırındaki çökme düzeltmesi; tamamlanmış blok imleci; blok menüsünde Paste; IME marked text.
- Vim `il` / `al`; Markdown görünümü değiştirilince scroll korunması; tekrarlanan discard için koruma; animasyonlu arka planın app tarafı.
- Ayarlarda boş kategori başlıklarının gizlenmesi, başlık yanına kontroller, satır numarası ayarının Editor and Code Review sayfasına taşınması, About ve settings pane yaşam döngüsü korumaları.
- Mevcut uygulama ikilisinden ayar şeması üretimi; eski channel-aware generator fallback'i korunur. Obsolete completion v2/plugin-host kodu kaldırıldı.
- Windows/Linux başlangıç izleyicisinde çift pencere önleme; mevcut winit bağımlılığının güncellenmesi; shell/build yardımcı düzeltmeleri.

Lite'ın yerel terminal odağı, kapalı AI/cloud/telemetry davranışı ve tmux düzeni korundu. `crates/warpui` ve `crates/warpui_core` başlangıçla byte-for-byte aynı. Dormant AI tüketicilerindeki bazı imza değişiklikleri yalnızca derleme uyarlamasıdır.

## Doğrulama

macOS üzerinde Rust 1.92.0 ile doğrulandı. Oturumun başlangıcındaki rustup metadata sorununu aşmak için doğrulanmış resmi toolchain `/tmp/warp-lite-sync-20261002/toolchain` altına kuruldu; global Rust ayarları değiştirilmedi. MetalToolchain bileşeni Xcode üzerinden indirildi.

| Kontrol | Sonuç |
| --- | --- |
| `cargo check -p warp --bin warp-oss` | Geçti (kod son noktasında) |
| `cargo test -p warp --lib --features agent_management_view --no-run` | Test ikilisi derlendi ve linklendi |
| `cargo test -p repo_metadata -p settings -p warp_completer -p warp_terminal -p warp_editor -p vim -p warp_util --lib` | **1.023 geçti, 9 ignored** |
| Seçili uygulama testleri, benzersiz test sayısı | **1.147 geçti, 1 başarısız**; aşağıdaki AI paneli beklentisi |
| Bash / zsh / fish bootstrap sözdizimi | Geçti |
| `script/lint_shellcheck`, `script/prepare_bundled_resources` Bash sözdizimi | Geçti |
| `git diff --check` ve korunan UI dizinleri diff'i | Temiz |

Uygulama test kapsamı: terminal modeli/ANSI/tmux (589), completer (22), native/fallback/handoff/input-tab (12), PTY command bytes (7), arama (28), sekmeler (10), workspace view (157 başarılı + 1 başarısız), launch config (16), ayarlar UI (59), şema (4), kod editörü (111), discard (1), terminal Vim (126), marked text (5). Aynı code-editor Vim testlerinin ikinci çalışması toplama dahil edilmedi.

Tek kalan başarısızlık: `workspace::view::tests::test_switch_focus_panels`, kapalı AI paneline odaklanma beklentisinde (`Expected AI panel to be focused`). `workspace/view.rs` AI paneli render yolunu mevcut `false && is_ai_assistant_panel_open` koşuluyla kapatır. Test ve `focus_left_panel` gövdesi başlangıç commitinde aynıdır; başlangıç için ayrı test ikilisi derlenip çalıştırılmadı. Bu nedenle tüm uygulama suite'inin yeşil olduğu iddia edilmiyor; AI panelini etkinleştiren bir değişiklik yapılmadı. `agent_management_view`, mevcut test fixture'larının derlenmesi için kullanıldı; ürünün varsayılan feature listesine eklenmedi.

Regresyonlar gerçekten çalıştırıldı: boş gitignore kaynaklarının 512 giriş sınırı, aynı dosya snapshot'ından fingerprint/parse, arama event sırası, kayıtlı olmayan shell seçim mesajı ve POSIX LF testi önce başarısız olup düzeltmeden sonra geçti. Cache testleri ortak cache temizleme yarışını önlemek için test kilidiyle seri hale getirildi. Standalone completer fixture'ları feature flag başlangıcını açıkça yapıyor.

Derleme existing cfg/dead-code uyarıları üretir; `-D warnings`/tam clippy geçişi iddia edilmez. ShellCheck ve PowerShell/PSScriptAnalyzer kurulu olmadığından lint/runtime doğrulanmadı. Windows, Linux, WSL guest ve PowerShell gerçek ortam testleri yapılmadı. Model/UI testleri native uygulamada görsel kontrolün yerine geçmez; yeni paket açılıp manuel test edilmedi. Release build, imzalama, notarization, kurulum, push veya release yayınlama yapılmadı.

Komut/sonuç özeti: [validation.json](docs/upstream-sync/2026-10-02-validation.json). Ham oturum logları `/tmp/warp-lite-sync-20261002/` altında; bu geçici dizin dağıtım artefaktı değildir.

## Bilinçli olarak alınmayanlar

- AI/Oz/cloud, teams, billing, sharing ve raporlama değişiklikleri Lite ürün kapsamı dışında.
- Upstream release/CI/agent otomasyonları, silinmiş GraphQL/voice backend bağımlılıkları ve WASM-only değişiklikler aktarılmadı.
- Korunan UI/core API gerektiren right-click Paste tercihi ve yardım metni, editor layout-cache bypass, headless platform refactor ve secret-redaction crate ayrıştırması tutuldu. TIFF clipboard, Windows RDP renderer, smart-select/glyph ve backdrop düzeltmeleri de bu sınır nedeniyle alınmadı.
- `21f413b79` terminal grid dosyalarını taşıyan refactor uygulanmadı; davranış yamaları Lite'ın mevcut yollarına uyarlandı.
- `114bacea6` rmcp 2.2 güncellemesi upstream 1.6 tabanını varsayıyor. Lite 0.10 git forkunu dormant tipler için tutuyor ve ilgili transport crate'leri silinmiş. Bu ayrı major bağımlılık geçişi yapılmadı; ilgili CVE'nin giderildiği veya Lite'ın etkilenmediği iddia edilmiyor.
- `200153270` public commitinin diff'i boş; bu committen bir düzeltme alındığı söylenemez.

## Alınan upstream kaynakları

Aşağıdaki yerel commitler yalnızca yerelde bulunur; uzak depoya gönderilmedi.

| Upstream | Değişiklik | Yerel commitler |
| --- | --- | --- |
| [33bb01256](https://github.com/warpdotdev/warp/commit/33bb01256937b48cd1c116ae5727c25e1f547e12) | Bump warp-command-signatures pin: add mpv/ruff completions, refresh deno (#15248) | `9ebdfe29b` |
| [c6609ef23](https://github.com/warpdotdev/warp/commit/c6609ef2364de6bf8e50ac907c352779ae9c6605) | APP-4828: share and cache gitignore matchers in repo_metadata file-tree traversal (#15240) | `c4edbd73d` |
| [213c9b32e](https://github.com/warpdotdev/warp/commit/213c9b32e374e6f2ad17c71d604692d56a4eb422) | Bound SignatureCache growth with a key-length cap and a bounded FIFO miss cache (APP-5431) (#15181) | `91f815dc9` |
| [8ba01aa1a](https://github.com/warpdotdev/warp/commit/8ba01aa1a8633932987799a21be44343d5170c6e) | Throttle grid_renderer out-of-bounds row report to once per run (#15288) | `f81f8610c` |
| [ee95ac0fd](https://github.com/warpdotdev/warp/commit/ee95ac0fd71803c4bfea60fea75aad56f2bdb1e4) | Fix double cursor in finished background blocks (CORE-3798) (#15322) | `a1f97111e` |
| [cff5f778c](https://github.com/warpdotdev/warp/commit/cff5f778cfb7b6cb04f1f245bfa405907959f03e) | lint_powershell: report findings from every source (plus Get-Location false-positive cleanup) (#15316) | `2f12213a0` |
| [4e49d04f5](https://github.com/warpdotdev/warp/commit/4e49d04f5a4dfa955245efc4985ef736116e0e42) | Follow symlinks to directories in WSL Tab completion, via guest-driven enumeration (APP-3993) (#14755) | `bc0f8a557` |
| [8b88df987](https://github.com/warpdotdev/warp/commit/8b88df9874d8d632eba3bcfdd330acc9dabd23b0) | Show switch-to-tab shortcut hints while their modifier is held (#15221) | `2223b958e`, `4a24ad66c` |
| [f42c4ab6c](https://github.com/warpdotdev/warp/commit/f42c4ab6c24a1b0d6c3485319cc83c4a83c62e67) | Compute UserBlockCompleted's expensive fields lazily (#15162) | `c05a9d38b` |
| [ff16a0b2a](https://github.com/warpdotdev/warp/commit/ff16a0b2af146684ca0deaff96a833b603289868) | Used faster hash maps and precomputed hashes in hot paths (#15279) | `9687584c0` |
| [36dd2cc2e](https://github.com/warpdotdev/warp/commit/36dd2cc2ecc0fbb3e9221b2aebb2e500f2405df1) | fix: coalesce AsyncSearcher full-index rebuilds to bound memory (APP-5389) (#15119) | `b20b8606a` |
| [092c1dce9](https://github.com/warpdotdev/warp/commit/092c1dce952f29264e8d281298e4574355fc7102) | Preserve scroll position when toggling markdown Rendered/Raw views (#13967) | `13810f60d` |
| [0a0fd3ae1](https://github.com/warpdotdev/warp/commit/0a0fd3ae10d9926840dbf259419af465d815ebb8) | Add Paste to the block list right-click context menu (#15346) | `9b45b3e82` |
| [3a7a4a5b3](https://github.com/warpdotdev/warp/commit/3a7a4a5b303a81f21f22764ed3ced962b45b7ae1) | Don't draw empty category headers before any search pass (APP-5559, PR0) (#15376) | `c768a30e0` |
| [0140af045](https://github.com/warpdotdev/warp/commit/0140af045df9a3bd791adb8a0649ebf11d93646b) | Fix zsh compadd shim dropping descriptions from _describe's clustered -ld (#15313) | `794b39818` |
| [a18026275](https://github.com/warpdotdev/warp/commit/a18026275e56442404492fcb631abe9429d6fa37) | Support trailing elements on settings page titles and category headers (APP-5559) (#15345) | `4823d469b` |
| [146684ee7](https://github.com/warpdotdev/warp/commit/146684ee7cfc10578bdb3a4233825c4fa191bc48) | Enable IME marked text on all platforms (Windows, Linux, macOS) (#15395) | `f81f8610c` |
| [90c2484dc](https://github.com/warpdotdev/warp/commit/90c2484dc0019b0625a047e6938a862ce6fc77cc) | Fix hidden, non-remappable Alt+1 Project Explorer binding (#15310) | `d464e4af1` |
| [78d1eabdf](https://github.com/warpdotdev/warp/commit/78d1eabdf02168de32990c6235c607a7a76663e5) | Fix self-invalidating rerun-if-changed for the Sentry framework path (#15433) | `c25d512f1` |
| [e722ebeda](https://github.com/warpdotdev/warp/commit/e722ebeda2862bcfe027a5bc38479c8d0397bd74) | Fix four latent shell-integration bugs (split out from native shell completions) (#15428) | `b3b3adbee`, `ec8fbb04a` |
| [83b4c101e](https://github.com/warpdotdev/warp/commit/83b4c101e425370f796bf484a46315834fc312b1) | Optimize settings schema generation during release process. (#13240) | `6519a04f0` |
| [6a96a72d8](https://github.com/warpdotdev/warp/commit/6a96a72d891552293164a2bee5eed030544a08f2) | Compile time: share settings registration code across settings (#15454) | `9557906b6` |
| [b1bcc3564](https://github.com/warpdotdev/warp/commit/b1bcc3564975cccc2971124b532fa2ac9161eb9e) | Add rust-analyzer to rust-toolchain.toml components (#15477) | `1a9bb3b0d` |
| [748b635cd](https://github.com/warpdotdev/warp/commit/748b635cd8e890f81c117735aca3cd793b5ce779) | Fix PSReadLine vi edit mode corrupting submitted commands in Warpified pwsh (#15476) | `159fb3337` |
| [79a9cb721](https://github.com/warpdotdev/warp/commit/79a9cb721aa05998a5d96167bfe7d8d430412e06) | completer: resolve an option's argument by value position, not the last declared (#15475) | `5921c73de` |
| [996babeea](https://github.com/warpdotdev/warp/commit/996babeeacceda63a4a1c28b356f9f1166482980) | Cite the ConPTY source for the PowerShell virtual key code claim (#15482) | `2223b958e` |
| [e326a774a](https://github.com/warpdotdev/warp/commit/e326a774a7a4ebb5829228f37eed866e58556a06) | [Completions] Bump command-signatures to 564724fe (#15480) | `9ebdfe29b` |
| [4b894db80](https://github.com/warpdotdev/warp/commit/4b894db8014523912617151094dc6a7741550a28) | Compile time: replace serde Content buffering with JSON-value deserializers (#15455) | `49bbc9869` |
| [e83d07d8b](https://github.com/warpdotdev/warp/commit/e83d07d8b7dcdfde9320b5750d8e323ed4b764a4) | [Completions] Bump command-signatures to d3725aa (#15487) | `9ebdfe29b` |
| [40e397170](https://github.com/warpdotdev/warp/commit/40e397170eb9998184b2c9d72dab573736b880e2) | Fix last-tab shortcut hint precedence (#15496) | `78e6ba0e6` |
| [d89e78385](https://github.com/warpdotdev/warp/commit/d89e783853596c082e6d9be5975aa2bfcb088739) | fix: stop cloning the whole file's styled blocks when loading a large file in the code editor (APP-4844) (#13508) | `e1781a611` |
| [e054075b8](https://github.com/warpdotdev/warp/commit/e054075b8715ffe6d8c6c17aef7e12c4d9c30afa) | Move Code editor line numbers setting to Editor and Code Review page (#15458) | `30cb3da58` |
| [1c925e333](https://github.com/warpdotdev/warp/commit/1c925e3330d7cbc9016d6a6cce59fe250bfd554d) | Bound rayon fan-out in EditDelta::layout_delta (APP-5392) (#15128) | `f9a327406` |
| [9589305a2](https://github.com/warpdotdev/warp/commit/9589305a257367c206e8ed9466fbb6920794aba2) | [Windows] Fix login-shell commands in PowerShell 5.1 (#15544) | `e2f7e6bdb` |
| [607be8c26](https://github.com/warpdotdev/warp/commit/607be8c26da633c882e53de2ab627730b6d28acc) | Fix bash bootstrap dropping shell_plugins from the Bootstrapped payload (#15518) | `a7c06f3f5` |
| [e36efd068](https://github.com/warpdotdev/warp/commit/e36efd06856227f31ce08d7e8df74e3ad549562b) | fix(workspace): Enable animation for theme background images (#14618) | `33aedae66` |
| [fc4d563b8](https://github.com/warpdotdev/warp/commit/fc4d563b8b6310e8533d98c5db762df484b87ad0) | Native shell completions: drive all shells through in-band generators (#15573) | `f2efb93e0` |
| [ccf683193](https://github.com/warpdotdev/warp/commit/ccf68319362d1ffec556df625fe264f0788bea28) | Remove unused orphaned cargo features (#15694) | `58bc27fa6` |
| [2e645f916](https://github.com/warpdotdev/warp/commit/2e645f916f7a2455bafe82a2a541f025692e1709) | Install protobuf in macOS bootstrap (#15710) | `c256f9751` |
| [83387d2ed](https://github.com/warpdotdev/warp/commit/83387d2ed13f220532c121c1f75dc3b3deda510a) | Pin winit to X11 request_user_attention panic fix (#15717) | `9ebdfe29b` |
| [e47c7ae9c](https://github.com/warpdotdev/warp/commit/e47c7ae9cd1350e34fe9bf43ef109e813f8603c7) | Replace hand-rolled test polling with assert_eventually (#15721) | `6e762c882` |
| [4fa1a3c66](https://github.com/warpdotdev/warp/commit/4fa1a3c664a9cd7bea951d2c62d2815846fd3af1) | History search: rank on history priors, not fuzzy score alone (APP-5650) (#15591) | `de040b141` |
| [bf2364bc9](https://github.com/warpdotdev/warp/commit/bf2364bc99c118f562e21b5529c1db492b0b939f) | Shell widget handoff: hand ctrl-r and ctrl-t to fzf/atuin (CORE-3807) (#15513) | `2223b958e`, `e687191a5` |
| [7c360f772](https://github.com/warpdotdev/warp/commit/7c360f772e33df9209da6626b37877a4cf71dd10) | Hand fish ctrl-r to PatrickF1/fzf.fish history search (GH 4125) (#15744) | `cc483498f` |
| [81744c2dc](https://github.com/warpdotdev/warp/commit/81744c2dc3c7c55fa07bb66f2d17b073b752c283) | Pin command-signatures so git ls-remote X-Ray describes --branches (#15756) | `9ebdfe29b` |
| [5cd24ed1b](https://github.com/warpdotdev/warp/commit/5cd24ed1bc3d5321ef18c4ce09f5734e61ebaacc) | [CORE-3810] Silence expected in-band command reset warning (#15779) | `329ed3686` |
| [83ddbefff](https://github.com/warpdotdev/warp/commit/83ddbefff792d6de515df4ef79a4c26052af21bf) | Complete git merge-base (#15769) | `9ebdfe29b` |
| [a7326f8fe](https://github.com/warpdotdev/warp/commit/a7326f8fecd6a52050a735da1e9979822b46c71b) | Fix crash when promoting an end-of-line cell to a wide char. (#15763) | `34d09025d` |
| [2a58ea28a](https://github.com/warpdotdev/warp/commit/2a58ea28a7f2fef3e5d6fecaa0efb02ab38f0ccf) | [APP-5776] Bump command-signatures for Rust Fig generators (#15791) | `9ebdfe29b` |
| [53b502c8e](https://github.com/warpdotdev/warp/commit/53b502c8eb42b0e3deae8b3a2c7baf5b28bdab31) | Make Create New Window an editable keybinding (#15771) | `d0dedc66c` |
| [17f432027](https://github.com/warpdotdev/warp/commit/17f4320276f9baaf272119f0c319db96a3aba183) | Fix double expansion of Bash PS1 in "honor PS1" mode (#15792) | `da3741217` |
| [77105899b](https://github.com/warpdotdev/warp/commit/77105899b8a15e7085834debfd1e0419e6602cb2) | Promote shell features to default (#15903) | `de040b141` |
| [6f575836c](https://github.com/warpdotdev/warp/commit/6f575836c02bd80a4b2de2755e952bec1793d3df) | fix(workspace): keep a new tab group's terminal from eating its name (#14895) | `5830c3a07` |
| [33c3bf6b7](https://github.com/warpdotdev/warp/commit/33c3bf6b7ecbb0755dd6f7583410e9a723bab572) | Guard duplicate Code Review discard confirmations (#15884) | `b5c02465d` |
| [3529ae637](https://github.com/warpdotdev/warp/commit/3529ae6373cf8e81c1aa182860b68b2a77fbe541) | Parent About page to Settings during tab transfer (#15948) | `010111358` |
| [4143c09ff](https://github.com/warpdotdev/warp/commit/4143c09ff8be80ff73165f36085cf4a013295a01) | Bump command signatures for du --time (#15950) | `9ebdfe29b` |
| [a06279712](https://github.com/warpdotdev/warp/commit/a06279712f838d01295575b0dc0f14b7a34ba049) | Rip out completions v2 and associated code. (#15965) | `98d71cb7b` |
| [c72f36d6a](https://github.com/warpdotdev/warp/commit/c72f36d6a91bf0733c02b4a2c1f1709f5aa7645f) | Save and restore tab groups in launch configurations (#14624) | `901a8b034` |
| [1bf1c6a2b](https://github.com/warpdotdev/warp/commit/1bf1c6a2b36bdbb977ad40a380c841182f2995fb) | fix: eliminate startup race in flaky async searcher rebuild test (#16064) | `5462e22b3` |
| [9c870be87](https://github.com/warpdotdev/warp/commit/9c870be87f9160121cf45d539aed356bf945eaef) | Fallback to file paths after empty native completions (#16093) | `ca286b432` |
| [0efd26e70](https://github.com/warpdotdev/warp/commit/0efd26e70c6ddfff1b5556261c978eaa19cad27a) | Add Neovim 0.13 line text objects (#16105) | `f463bdb69` |
| [64f652a8f](https://github.com/warpdotdev/warp/commit/64f652a8f720156ebbca587e02c551f4613e3d6a) | Add conservative ShellCheck gate for Bash bootstrap (#16104) | `711bc5fa8` |
| [f2b401aed](https://github.com/warpdotdev/warp/commit/f2b401aed528ee27c80d062183137f7ffe6e6248) | Support fzf Alt-C across shell integrations (#16094) | `52a2daf33` |
| [d5fdbff50](https://github.com/warpdotdev/warp/commit/d5fdbff502b261032beb6db41748cc14418b6723) | Fix flag completions after POSIX double dash (#16124) | `7f5c38279` |
| [b42435622](https://github.com/warpdotdev/warp/commit/b4243562280756b8a80b14d902f747dea050a4b8) | Constrain external Ctrl-R fzf history picker layout (#16123) | `acfaf2fde` |
| [2120e3794](https://github.com/warpdotdev/warp/commit/2120e3794710027577f731e4b8ce4814405f3c53) | Forward typed query to zsh fzf Ctrl-R history search (#16122) | `1ece0f3f1` |
| [43eae5e08](https://github.com/warpdotdev/warp/commit/43eae5e082db7991da89388d57125df6ae9a130d) | Restore focus after closing tab context menus (#16138) | `95c107163` |
| [9fb32eddd](https://github.com/warpdotdev/warp/commit/9fb32eddde1c64266c5d6f737aafee953fcc340b) | Make vertical tabs search match and highlight tab group names (#14690) | `e2481bdfa` |
| [7cef787ea](https://github.com/warpdotdev/warp/commit/7cef787ea0e14073c907e7628ddb8d1a720222b5) | warp-issues-6004:Two windows are openend when starting in Windows  #6004 (#16072) | `9771709f7` |
| [6d6eac300](https://github.com/warpdotdev/warp/commit/6d6eac300cc575457ee713bb1281db53d6c14177) | Fallback to file paths after empty combined completions (#16213) | `30db781d3` |
| [40b791c35](https://github.com/warpdotdev/warp/commit/40b791c351377c4b30c48179a568326144113726) | Avoid WSL chip subprocesses and expand session-home paths (#16165) | `92fe0ede4` |
| [3308af3ac](https://github.com/warpdotdev/warp/commit/3308af3ac71798197c9d6af838b43c62c03a569c) | [APP-4553] Skip undo-close cleanup for unavailable pane groups (#16251) | `32604792c` |
| [71cbee776](https://github.com/warpdotdev/warp/commit/71cbee776ef55716b2993383aa83fd19234276e4) | [APP-6223] Skip settings-error sync for unavailable panes (#16249) | `b5c02465d` |

Ek ön koşul: [dc2155193](https://github.com/warpdotdev/warp/commit/dc21551936ce85b0cb3945cb83705a3aadd60b38), yerel `b3b3adbee`.
