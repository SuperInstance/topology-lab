# topology-lab

**Interactive topological data analysis in your browser — persistent homology, symplectic integrators, sheaf cohomology on I Ching hexagrams, and music tension curves, all running client-side via Rust → WASM.**

Built with Leptos (Rust WASM framework). Draw point clouds, compute Vietoris-Rips filtrations, watch persistence diagrams and Betti curves appear in real-time. Compare symplectic integrators. Cast I Ching readings with live H⁰/H¹ sheaf cohomology. Explore chord progression tension.

## What This Gives You

- **Persistence Lab** — draw points → Vietoris-Rips filtration → persistence diagrams + Betti curves in real-time
- **Symplectic Playground** — compare Euler vs Verlet vs Yoshida integrators on harmonic oscillator, pendulum, Kepler orbit
- **I Ching Oracle** — cast readings with live sheaf cohomology (H⁰ = agreement, H¹ = wisdom)
- **Music Explorer** — tension curves for ii-V-I, Pachelbel, Tristan, Coltrane progressions
- **Social Networks** — ER, BA, WS random graphs with topological fingerprints
- **Conjecture Board** — 10 conjectures with evidence status from the SuperInstance project
- **Pure client-side math** — everything runs in WASM, no server needed

## Quick Start

```bash
# Install trunk (Leptos build tool)
cargo install trunk

# Build and serve
trunk serve --open
```

## Architecture

```
src/
├── math/
│   ├── simplex.rs       # Simplicial complex + Vietoris-Rips
│   ├── persistence.rs   # Persistent homology (H⁰, H¹)
│   ├── sheaf.rs         # Cellular sheaf cohomology
│   ├── symplectic.rs    # Hamiltonian integrators
│   ├── music.rs         # Chord tension + voice leading
│   └── iching.rs        # 64 hexagrams + cohomology mapping
├── components/          # Canvas-based visualizations
│   ├── persistence_diagram.rs
│   ├── betti_curve.rs
│   ├── simplicial_complex.rs
│   ├── phase_space.rs
│   ├── tension_curve.rs
│   └── hexagram_view.rs
└── pages/               # Route pages
```

## Key Experiments

- **Conservation of Tension** (confirmed) — dT/dt is conserved in well-formed chord progressions
- **H¹ as Wisdom Measure** (confirmed) — H¹ of I Ching readings correlates with depth ratings at r=0.927
- **Barabási-Albert Signature** (confirmed) — scale-free networks have distinctive persistence diagram signatures

## Installation

```bash
cargo build --target wasm32-unknown-unknown --release
# or use trunk for development
trunk serve
```

## How It Fits

Part of the SuperInstance ecosystem:

- **[persistent-sheaf](https://github.com/SuperInstance/persistent-sheaf)** — Rust library for persistent sheaf cohomology
- **[persistent-social](https://github.com/SuperInstance/persistent-social)** — TDA for social networks in Go
- **topology-lab** — Interactive WASM visualization (this repo)

## License

Apache-2.0
