#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

# Define targets
LINUX_TARGET="x86_64-unknown-linux-gnu"
WINDOWS_TARGET="x86_64-pc-windows-gnu"

# Output directory
OUTPUT_DIR="target/releases"
TITLED_DB="titledb.db"

# Ensure the titledb.db file exists
if [[ ! -f $TITLED_DB ]]; then
    echo "Error: $TITLED_DB not found."
    exit 1
fi

# Extract the package version from Cargo.toml
PACKAGE_VERSION=$(cargo metadata --format-version 1 | jq -r '.packages[0].version')

# Ensure the version was extracted
if [[ -z "$PACKAGE_VERSION" ]]; then
    echo "Error: Failed to extract package version from Cargo.toml."
    exit 1
fi

mkdir -p $OUTPUT_DIR

# Function to copy binary and titledb.db, then create a tarball
copy_files_and_create_tar() {
    local binary_path=$1
    local output_name=$2
    local platform=$3

    # Create a directory for the release files
    local release_dir="$OUTPUT_DIR/$output_name"
    mkdir -p "$release_dir"

    # Copy the binary and titledb.db to the release directory
    cp "$binary_path" "$release_dir/"
    cp "$TITLED_DB" "$release_dir/"

    # Create a tarball with the version number in the filename
    tar -czf "$OUTPUT_DIR/$output_name-v$PACKAGE_VERSION.tar.gz" -C "$OUTPUT_DIR" "$output_name"
    
    # Clean up the release directory after creating the tarball
    rm -rf "$release_dir"
}

echo "Building for Linux..."
cargo build --release --target $LINUX_TARGET
copy_files_and_create_tar "target/$LINUX_TARGET/release/hshopper" "hshopper-linux" "linux"

echo "Building for Windows..."
cargo build --release --target $WINDOWS_TARGET
copy_files_and_create_tar "target/$WINDOWS_TARGET/release/hshopper.exe" "hshopper-windows.exe" "windows"

echo "Builds completed. Output tarballs are in $OUTPUT_DIR"
