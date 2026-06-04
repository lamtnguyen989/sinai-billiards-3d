#!/usr/bin/env bash

set -euo pipefail

# Peg a slang version and install directory
SLANG_VERSION="${SLANG_VERSION:-2026.10.2}"  # Latest as of the time of writing
SLANG_DIR="${SLANG_DIR:-$HOME/.local/slang}"
mkdir -p "$SLANG_DIR"

# Download slangc 
ARCHIVE="slang-${SLANG_VERSION}-linux-x86_64.tar.gz"
URL="https://github.com/shader-slang/slang/releases/download/v${SLANG_VERSION}/${ARCHIVE}"
curl -fsSL "$URL" -o /tmp/slang.tar.gz

# Unpack slangc binary
tar -xzf /tmp/slang.tar.gz -C "$SLANG_DIR" --strip-components=1

# Path resolving after install
echo "${SLANG_DIR}" >> "$GITHUB_PATH"
export PATH="${SLANG_DIR}:$PATH"