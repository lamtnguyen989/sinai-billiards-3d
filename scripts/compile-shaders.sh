#!/usr/bin/env bash

SHADER_DIR="src/shaders"
SLANG_FILE="shaders.slang"
TARGET="${1:-wgsl}"

SLANG_FLAGS="-O3 -whole-program -fp-mode precise"

slangc $SLANG_FLAGS -target $TARGET "$SHADER_DIR/$SLANG_FILE" -o "$SHADER_DIR/shaders.$TARGET"

sed -i "1i /*** Compiled shaders from Slang ***/ \n" "$SHADER_DIR/shaders.$TARGET"