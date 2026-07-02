#!/usr/bin/env bash

# Source file 
SHADER_DIR="src/shaders"
SLANG_FILE="shaders.slang"

# Compilation target
TARGET="${1:-wgsl}"
EXT=$TARGET

if [ "spirv" == "$TARGET" ]; then 
    EXT="spv"
fi

# Compiliation flags
SLANG_FLAGS="-O3 -whole-program -fp-mode precise"

# Compile shaders
slangc $SLANG_FLAGS -target $TARGET "$SHADER_DIR/$SLANG_FILE" -o "$SHADER_DIR/shaders.$EXT"
