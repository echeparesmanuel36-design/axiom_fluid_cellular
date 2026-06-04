# 🪐 AXIOM_FLUID_CELLULAR // Real-Time Fluid Dynamics Engine v1.0

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

**High-velocity, single-threaded cellular particle fluid simulation mechanics.**

`axiom_fluid_cellular` is an ultra-lightweight physics engine core written in pure **Rust**. It simulates gravity, viscosity vectors, bounding collisions, and dynamic friction for thousands of independent liquid particles simultaneously directly on the CPU, achieving flawless frame rates without heavy grid structures.

---

## ⚡ Fluid Specs

* **🌊 Viscosity Vectoring:** Custom continuous velocity scattering algorithm for realistic liquid accumulation behavior.
* **🔌 Zero Setup:** Standard plug-and-play architecture. No CUDA or heavy GPU compute shaders required.
* **🪐 Bare-Metal Efficiency:** Packs thousands of molecular liquid points into packed memory arrays for optimal data flow.

---

## 🚀 Run in 5 Seconds

1. **Clone the matrix:**
   ```bash
   git clone [https://github.com/echeparesmanuel36-design/axiom_fluid_cellular.git](https://github.com/echeparesmanuel36-design/axiom_fluid_cellular.git)
   cd axiom_fluid_cellular
   ```
2. **Run:**
```bash
cargo run --release
 ```
Developed by Axiom Systems. ⚡
