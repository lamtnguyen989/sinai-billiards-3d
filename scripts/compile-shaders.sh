#!/usr/bin/env bash

SHADER_DIR="src/shaders"
SLANG_FILE="shaders.slang"
TARGET="wgsl"

slangc "$SHADER_DIR/$SLANG_FILE" -o "$SHADER_DIR/shaders.$TARGET"