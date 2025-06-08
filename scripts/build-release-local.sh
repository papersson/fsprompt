#!/bin/bash
# Build release artifacts locally

set -e

echo "Building fsPrompt release artifacts locally..."

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo "Version: $VERSION"

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" = "Darwin" ]; then
    # macOS build
    if [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
    
    echo "Building for macOS ($TARGET)..."
    cargo build --release --target $TARGET
    
    # Build .pkg installer
    echo "Creating macOS installer..."
    ./scripts/packaging/build-macos-pkg.sh
    
    # Also create tar.gz
    cd target/$TARGET/release
    tar czf ../../../fsprompt-v$VERSION-$TARGET.tar.gz fsprompt
    cd ../../..
    
elif [ "$OS" = "Linux" ]; then
    # Linux build
    TARGET="x86_64-unknown-linux-gnu"
    echo "Building for Linux..."
    cargo build --release --target $TARGET
    
    cd target/$TARGET/release
    tar czf ../../../fsprompt-v$VERSION-$TARGET.tar.gz fsprompt
    cd ../../..
else
    echo "Unsupported OS: $OS"
    exit 1
fi

echo "Build complete! Artifacts:"
ls -la fsprompt-v$VERSION-*