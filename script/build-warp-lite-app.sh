#!/usr/bin/env bash
# Builds a minimal WarpLite.app bundle on macOS for warp-lite.
#
# Prerequisites:
#   - target/release/warp-oss already built (cargo build --release --bin warp-oss)
#
# Usage:
#   script/build-warp-lite-app.sh
#
# Output:
#   ./WarpLite.app   (drag into /Applications)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BIN="target/release/warp-oss"
APP="WarpLite.app"
APP_VERSION="0.3.1"
APP_SHORT_VERSION="0.3.1-lite"
APP_IDENTIFIER="dev.warp-lite.WarpLite"
SRC_PNG="app/channels/oss/icon/no-padding/512x512.png"

if [[ ! -f "$BIN" ]]; then
    echo "Error: $BIN not found. Run: cargo build --release --bin warp-oss" >&2
    exit 1
fi

if [[ ! -f "$SRC_PNG" ]]; then
    echo "Error: source icon $SRC_PNG not found." >&2
    exit 1
fi

# Wipe any prior bundle
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

# 1) Copy the binary
cp "$BIN" "$APP/Contents/MacOS/warp-oss"
chmod +x "$APP/Contents/MacOS/warp-oss"

# 2) Generate AppIcon.icns from the 512×512 source via iconset
ICONSET="$(mktemp -d)/AppIcon.iconset"
mkdir -p "$ICONSET"
sips -z 16   16   "$SRC_PNG" --out "$ICONSET/icon_16x16.png"        >/dev/null
sips -z 32   32   "$SRC_PNG" --out "$ICONSET/icon_16x16@2x.png"     >/dev/null
sips -z 32   32   "$SRC_PNG" --out "$ICONSET/icon_32x32.png"        >/dev/null
sips -z 64   64   "$SRC_PNG" --out "$ICONSET/icon_32x32@2x.png"     >/dev/null
sips -z 128  128  "$SRC_PNG" --out "$ICONSET/icon_128x128.png"      >/dev/null
sips -z 256  256  "$SRC_PNG" --out "$ICONSET/icon_128x128@2x.png"   >/dev/null
sips -z 256  256  "$SRC_PNG" --out "$ICONSET/icon_256x256.png"      >/dev/null
sips -z 512  512  "$SRC_PNG" --out "$ICONSET/icon_256x256@2x.png"   >/dev/null
sips -z 512  512  "$SRC_PNG" --out "$ICONSET/icon_512x512.png"      >/dev/null
cp "$SRC_PNG" "$ICONSET/icon_512x512@2x.png"
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"
rm -rf "$(dirname "$ICONSET")"

# 3) Info.plist
cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>WarpLite</string>
    <key>CFBundleDisplayName</key>
    <string>warp-lite</string>
    <key>CFBundleIdentifier</key>
    <string>$APP_IDENTIFIER</string>
    <key>CFBundleVersion</key>
    <string>$APP_VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$APP_SHORT_VERSION</string>
    <key>CFBundleExecutable</key>
    <string>warp-oss</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>WLIT</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSHumanReadableCopyright</key>
    <string>Forked from Warp Terminal © Denver Technologies, Inc. — AGPL-3.0-only</string>
</dict>
</plist>
EOF

# 4) Ad-hoc codesign so Gatekeeper does not refuse the binary outright.
codesign --force --deep --sign - "$APP" >/dev/null

echo "Built $APP ($(du -sh "$APP" | awk '{print $1}'))"
echo "  Identifier: $APP_IDENTIFIER"
echo "  Version:    $APP_SHORT_VERSION"
echo "Drag $APP into /Applications, or run: open $APP"
