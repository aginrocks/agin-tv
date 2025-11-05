#!/bin/bash

# Test script for Agin TV Desktop
# This script runs the built application with the required environment variable

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/../target/release/desktop"

echo "🚀 Testing Agin TV Desktop Application"
echo "======================================"
echo ""

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found at: $BINARY_PATH"
    echo ""
    echo "Please build the application first:"
    echo "  pnpm tauri:build"
    echo ""
    exit 1
fi

echo "✅ Binary found: $BINARY_PATH"
echo "🔧 Setting WEBKIT_DISABLE_DMABUF_RENDERER=1"
echo "▶️  Launching application..."
echo ""

# Run the application with the required environment variable
export WEBKIT_DISABLE_DMABUF_RENDERER=1
exec "$BINARY_PATH" "$@"
