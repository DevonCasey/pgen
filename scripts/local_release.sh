#!/usr/bin/env bash
set -e

# This script builds the RPM and Windows executable locally.
# Requirements:
# - For RPM: Fedora with required packages, cargo-generate-rpm
# - For Windows exe: Rust with x86_64-pc-windows-msvc target and wine (optional for testing)

DIST_DIR="dist"
mkdir -p "$DIST_DIR"

# --- RPM Build ---
echo "Installing RPM build dependencies (Fedora only)"
sudo dnf update -y
sudo dnf install -y git curl gcc rpm-build rpm-devel libtool make libxcb-devel

if ! command -v cargo-generate-rpm &> /dev/null; then
    echo "Installing cargo-generate-rpm..."
    cargo install cargo-generate-rpm
fi

echo "Building release and generating RPM"
cargo build --release
cargo generate-rpm

VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d '"' -f2)
RPM_FILE=$(find target/generate-rpm -name "*.rpm" | head -1)
NEW_NAME="pgen-${VERSION}.rpm"

if [ -z "$RPM_FILE" ]; then
    echo "RPM file not found!" >&2
    exit 1
fi

# Only rename if it doesn't already have the correct name
if [ "$(basename "$RPM_FILE")" != "$NEW_NAME" ]; then
    mv "$RPM_FILE" "target/generate-rpm/$NEW_NAME"
    echo "Renamed RPM to $NEW_NAME"
else
    echo "RPM file already has the correct name: $NEW_NAME"
fi

cp "target/generate-rpm/$NEW_NAME" "$DIST_DIR/"
echo "RPM package copied to $DIST_DIR/$NEW_NAME"

echo "All distribution files are in $DIST_DIR/"