#!/bin/bash
set -e  # Exit on error

# Step 1: Build the Rust project (release version)
echo "Building the release binary..."
cargo build --release

# Step 2: Move the binary to /usr/local/bin
echo "Installing the binary as 'bastion'..."
sudo cp target/release/view /usr/local/bin/bastion #adds view binary to $PATH as bastion

# Step 3: Verify installation
if command -v bastion >/dev/null 2>&1; then
    echo "Installation successful! You can now run 'bastion' from anywhere."
else
    echo "Installation failed. Please check your PATH or permissions."
fi
