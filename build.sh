
set -e  # Exit on error

# Step 1: Run the cargo build command
cargo build --release

# Step 2: Copy the binary to /usr/local/bin
echo "Installing the binary as 'bastion'..."
sudo cp target/release/view /usr/local/bin/bastion
