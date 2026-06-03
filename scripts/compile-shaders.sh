#!/usr/bin/env bash

SHADER_DIR="src/shaders"
SLANG_FILE="shaders.slang"
TARGET="wgsl"

SLANG_FLAGS="-O3 -whole-program"

slangc $SLANG_FLAGS "$SHADER_DIR/$SLANG_FILE" -o "$SHADER_DIR/shaders.$TARGET"