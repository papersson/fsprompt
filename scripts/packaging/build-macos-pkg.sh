#!/bin/bash
set -e

# Build macOS .pkg installer for fsPrompt
# This script creates an unsigned package that can be distributed

VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
ARCH=$(uname -m)

if [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-apple-darwin"
else
    TARGET="x86_64-apple-darwin"
fi

echo "Building fsPrompt v$VERSION for $TARGET..."

# Build release binary
cargo build --release --target $TARGET

# Create package structure
PKG_ROOT="pkg_build"
rm -rf $PKG_ROOT
mkdir -p $PKG_ROOT/usr/local/bin

# Copy binary
cp target/$TARGET/release/fsprompt $PKG_ROOT/usr/local/bin/
chmod 755 $PKG_ROOT/usr/local/bin/fsprompt

# Create package
pkgbuild --root $PKG_ROOT \
         --identifier com.fsprompt.app \
         --version $VERSION \
         --install-location / \
         --scripts scripts/packaging/macos-scripts \
         fsprompt-v$VERSION-$TARGET.pkg

# Clean up
rm -rf $PKG_ROOT

echo "Package created: fsprompt-v$VERSION-$TARGET.pkg"
echo "Users will need to right-click and select 'Open' to bypass Gatekeeper"