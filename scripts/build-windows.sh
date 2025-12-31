#!/bin/bash
# Build Windows release using cross-compilation
# Requires: mingw-w64-gcc toolchain

set -e

VERSION="0.1.0"
TARGET="x86_64-pc-windows-gnu"
RELEASE_DIR="akkurate-$VERSION-windows-x86_64"

echo "Checking Windows cross-compilation target..."
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "Installing Windows target..."
    rustup target add "$TARGET"
fi

echo "Building for Windows..."
cargo build --release --target "$TARGET"

echo "Creating release directory..."
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

echo "Copying files..."
cp "target/$TARGET/release/akkurate.exe" "$RELEASE_DIR/"
cp -r assets "$RELEASE_DIR/"
cp README.md "$RELEASE_DIR/"

# Create batch file launcher
cat > "$RELEASE_DIR/akkurate.bat" << 'EOF'
@echo off
cd /d "%~dp0"
akkurate.exe %*
EOF

echo "Creating zip..."
zip -r "$RELEASE_DIR.zip" "$RELEASE_DIR"

echo ""
echo "✅ Windows release created: $RELEASE_DIR.zip"
echo ""
echo "Users can:"
echo "  1. Extract the zip"
echo "  2. Run akkurate.exe"
