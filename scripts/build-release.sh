#!/bin/bash
# Build portable release for Linux

set -e

VERSION="0.1.0"
RELEASE_DIR="akkurate-$VERSION-linux-x86_64"

echo "Building release..."
cargo build --release

echo "Creating release directory..."
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

echo "Copying files..."
cp target/release/akkurate "$RELEASE_DIR/"
cp -r assets "$RELEASE_DIR/"
cp README.md "$RELEASE_DIR/"

# Create run script
cat > "$RELEASE_DIR/run.sh" << 'EOF'
#!/bin/bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/akkurate" "$@"
EOF
chmod +x "$RELEASE_DIR/run.sh"

# Create install script
cat > "$RELEASE_DIR/install.sh" << 'EOF'
#!/bin/bash
set -e
INSTALL_DIR="${1:-$HOME/.local}"

echo "Installing Akkurate to $INSTALL_DIR..."
install -Dm755 akkurate "$INSTALL_DIR/bin/akkurate"

echo "Done! Add $INSTALL_DIR/bin to your PATH if not already."
echo "Configure hotkey: akkurate -s"
EOF
chmod +x "$RELEASE_DIR/install.sh"

echo "Creating tarball..."
tar -czvf "$RELEASE_DIR.tar.gz" "$RELEASE_DIR"

echo ""
echo "✅ Release created: $RELEASE_DIR.tar.gz"
echo ""
echo "Users can:"
echo "  1. Extract and run: ./akkurate"
echo "  2. Install: ./install.sh"
echo "  3. Configure hotkey: akkurate -s"
