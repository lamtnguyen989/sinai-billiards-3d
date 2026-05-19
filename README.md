# 3D Sinai Billiards Ergodic Dynamics

This is my repository for learning and computing Ergodic Theory concepts and Rust graphics programming through WebGPU. Naturally, as this is mainly a self-explore effort, I plan for this to be a long-term project that I am developing while actively learning [ergodic theory](https://en.wikipedia.org/wiki/Ergodic_theory).

## Plan
While this is way down the line, in the end I want this to be a real-time render of the 3D version of the [Sinai billiards](https://en.wikipedia.org/wiki/Dynamical_billiards#Lorentz_gas,_a.k.a._Sinai_billiard) and computations of ergodic quantities such as 
- [Lyapunov spectra](https://en.wikipedia.org/wiki/Lyapunov_exponent)
- Entropies (most likely just [KS](https://mathworld.wolfram.com/KolmogorovEntropy.html) but if I find a way to derive the [topological entropy](https://en.wikipedia.org/wiki/Topological_entropy) numerically then it would be great)

_Computations for trajectory physics on CPU should be fine since we are only ever do computations for 1 path. GPU could just be used for rendering._