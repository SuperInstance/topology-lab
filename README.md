# topology-lab

**Interactive Mathematical Visualization Lab — the flagship frontend for the [SuperInstance](https://github.com/SuperInstance) ecosystem.**

Runs entirely in the browser via Rust → WASM (Leptos framework). No server, no backend, no install. Six interactive labs where you can touch the math.

## 🧪 Labs

| Lab | What You Do |
|-----|-------------|
| **Persistence Homology** | Draw points → watch Vietoris-Rips filtration build in real-time. See Betti numbers change as the radius grows. |
| **Symplectic Playground** | Compare Euler vs Verlet vs Yoshida integrators on phase space. Watch orbits drift or conserve in real-time. |
| **I Ching Oracle** | Cast hexagram readings with live sheaf cohomology computation. The 64 hexagrams as a simplicial complex. |
| **Music Explorer** | Visualize tension gradients of chord progressions. See ii-V-I vs random sequences ranked by CR. |
| **Social Networks** | Generate random networks (Erdős-Rényi, Barabási-Albert), compute persistence, compare topologies. |
| **Conjecture Board** | Live scoreboard of 10 mathematical conjectures being tested across the ecosystem. |

## 🚀 Quick Start

```bash
# Prerequisites
cargo install cargo-leptos

# Development server with hot reload
cargo leptos watch

# Production build
cargo leptos build    # → target/site/
```

Open `http://localhost:3000` and start clicking.

## 🧮 Pure Rust Math

All computation is client-side, `no_std`-friendly Rust compiled to WASM:

- **Vietoris-Rips filtration** & persistent homology (union-find for H₀, boundary reduction for H₁)
- **Symplectic Euler**, **Störmer-Verlet**, **Yoshida 4th-order** integrators
- **Spectral & voice-leading tension** computation from chord progressions
- **Sheaf cohomology** of I Ching readings
- **Conservation ratio** (CR = λ₂/λ_max) for any graph you draw

No external APIs. No telemetry. Everything runs in your browser.

## How It Fits

topology-lab is the interactive face of the SuperInstance spectral ecosystem:

- **[spectral-graph-core](https://github.com/SuperInstance/spectral-graph-core)** — The Rust engine behind the graph labs
- **[sheaf-cohomology](https://github.com/SuperInstance/sheaf-cohomology)** — The I Ching cohomology engine
- **[symplectic-spin](https://github.com/SuperInstance/symplectic-spin)** — The integrators compared in the playground
- **[spectral-music-v2](https://github.com/SuperInstance/spectral-music-v2)** — The music theory behind the explorer
- **[topological-flow](https://github.com/SuperInstance/topological-flow)** — The persistence engine

## Testing

```bash
cargo test
```

## License

MIT

Part of the [SuperInstance](https://github.com/SuperInstance) ecosystem.
