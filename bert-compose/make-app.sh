#!/usr/bin/env bash
# Build and install "BERT Compose.app" — same recipe as hal-console/germen:
# a real bundle so Dock, Spotlight, and UI automation see it.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --release -p bert-compose

DEST="${1:-/Applications}"
BUNDLE="$DEST/BERT Compose.app"

rm -rf "$BUNDLE"
mkdir -p "$BUNDLE/Contents/MacOS"
cp target/release/bert-compose "$BUNDLE/Contents/MacOS/bert-compose"

cat > "$BUNDLE/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>BERT Compose</string>
    <key>CFBundleDisplayName</key>
    <string>BERT Compose</string>
    <key>CFBundleIdentifier</key>
    <string>com.halcyonic.bert-compose</string>
    <key>CFBundleExecutable</key>
    <string>bert-compose</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
PLIST

codesign --force -s - "$BUNDLE"
echo "installed: $BUNDLE"
