# topology-lab

**Interactive Mathematical Visualization Lab** — the flagship frontend for the [SuperInstance](https://github.com/SuperInstance) ecosystem.

Runs entirely in the browser via Rust → WASM (Leptos framework). No server needed.

## 🧪 Labs

| Lab | What You Do |
|-----|-------------|
| **Persistence Homology** | Draw points → watch Vietoris-Rips filtration build in real-time |
| **Symplectic Playground** | Compare Euler vs Verlet vs Yoshida integrators on phase space |
| **I Ching Oracle** | Cast readings with live sheaf cohomology computation |
| **Music Explorer** | Visualize tension gradients of chord progressions |
| **Social Networks** | Generate random networks, compute persistence, compare topologies |
| **Conjecture Board** | Live scoreboard of 10 mathematical conjectures |

## 🚀 Build

```bash
cargo install cargo-leptos
cargo leptos watch       # dev server
cargo leptos build       # production build → target/site/
```

## 🧮 Pure Rust Math

All computation is client-side, `no_std`-friendly Rust:
- Vietoris-Rips filtration & persistent homology (union-find H₀, boundary reduction H₁)
- Symplectic Euler, Störmer-Verlet, Yoshida 4th-order integrators
- Spectral & voice-leading tension computation
- Sheaf cohomology of I Ching readings

## License

MIT
