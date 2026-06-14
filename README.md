# 3D Sinai Billiards Ergodic Dynamics

A live rendering and ergodic statistics computation to a 3D extension of the [Sinai billiards](https://en.wikipedia.org/wiki/Dynamical_billiards#Lorentz_gas,_a.k.a._Sinai_billiard) Dynamical System with Rust and WebGPU.

![image info](assets/rendering.png)

## Requirements and Usage
- Install `rust` via [rustup](https://rust-lang.org/tools/install/).
- Install a [release](https://github.com/shader-slang/slang/releases/tag/v2026.10.2) of a `slangc` shader compiler for the Slang shading language.
- (If you made update to the Slang [shader source](src/shaders/shaders.slang)) Compile the shaders to WGSL with the `scripts/compile-shaders.sh`.
- Build and run with `cargo run --release`.
