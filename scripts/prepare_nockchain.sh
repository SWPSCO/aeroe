#!/bin/bash
set -e # Exit on error

# Ensure submodule is initialized and updated
# Developers run this after cloning the main repo
# git submodule update --init --recursive

echo "Navigating to submodule directory..."
# Assuming this script is run from the aeroe root
cd vendor/nockchain || exit 1

# Optional: Check out a specific commit/branch/tag if needed for consistency
# This rev matches the one previously in Cargo.toml
TARGET_REV="314c5ebdf1c354d1620e8e24f341f44658e52128"
echo "Fetching latest refs and checking out revision $TARGET_REV..."
git fetch origin
git checkout "$TARGET_REV"

# Run the make commands to generate assets
echo "Running make commands in nockchain..."
# 1. Install choo
echo "Running make install-hoonc..."
if make install-hoonc; then
  echo "make install-hoonc completed successfully."
else
  echo "make install-hoonc failed."
  exit 1
fi

# 2. Build hoon artifacts (which likely produces the .jam files)
echo "Running make build-hoon-all..."
if make build-hoon-all; then
  echo "make build-hoon-all completed successfully."
else
  echo "make build-hoon-all failed."
  exit 1
fi

echo "Nockchain prepared."
cd ../.. # Return to aeroe root 
