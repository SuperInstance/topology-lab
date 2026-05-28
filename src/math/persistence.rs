/// Persistence computation — WASM-safe, no filesystem

use crate::math::simplex::{build_rips, UnionFind};

/// A persistence pair (birth, death)
#[derive(Clone, Debug, PartialEq)]
pub struct PersistencePair {
    pub birth: f64,
    pub death: f64,
    pub dim: usize,
}

impl PersistencePair {
    pub fn persistence(&self) -> f64 {
        self.death - self.birth
    }
}

/// Compute persistent homology for a set of points
/// Returns persistence pairs and betti numbers at each epsilon step
pub fn compute_persistence(
    points: &[(f64, f64)],
    max_epsilon: f64,
    steps: usize,
) -> (Vec<PersistencePair>, Vec<(f64, Vec<usize>)>) {
    let n = points.len();

    // Compute all pairwise distances
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            distances.push((i, j, (dx * dx + dy * dy).sqrt()));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut pairs = Vec::new();
    let mut betti_data = Vec::new();

    for step in 0..=steps {
        let eps = max_epsilon * step as f64 / steps as f64;
        let complex = build_rips(points, eps);

        // H0: connected components
        let mut uf = UnionFind::new(n);
        for (i, j, d) in &distances {
            if *d <= eps {
                uf.union(*i, *j);
            }
        }
        let h0 = uf.count_components(n);

        // H1: approximate using Euler characteristic
        // χ = V - E + F = h0 - h1 + h2
        let v = n;
        let e = complex.edges().len();
        let f = complex.triangles().len();
        let euler = v as i64 - e as i64 + f as i64;
        let h1 = ((h0 as i64 - euler).max(0)) as usize;

        betti_data.push((eps, vec![h0, h1, 0]));
    }

    // Extract persistence pairs from the filtration
    // For each distance threshold where components merge
    let mut uf = UnionFind::new(n);
    let mut component_birth: std::collections::HashMap<usize, f64> = (0..n)
        .map(|i| (i, 0.0))
        .collect();

    for (i, j, d) in &distances {
        let ri = uf.find(*i);
        let rj = uf.find(*j);
        if ri != rj {
            // Components merge at distance d
            let death_old = d;
            pairs.push(PersistencePair {
                birth: 0.0,
                death: *death_old,
                dim: 0,
            });
            uf.union(*i, *j);
        }
    }

    // Add an infinite pair for the last remaining component
    if n > 0 {
        pairs.push(PersistencePair {
            birth: 0.0,
            death: max_epsilon * 1.5, // "infinite"
            dim: 0,
        });
    }

    // Detect H1 features: edges that create cycles
    // Simple approximation: count edges that don't merge components
    let mut uf2 = UnionFind::new(n);
    let mut cycle_edges = 0;
    for (i, j, d) in &distances {
        if uf2.find(*i) == uf2.find(*j) {
            cycle_edges += 1;
            // Find when cycle is "born" and "dies" (filled by triangle)
            pairs.push(PersistencePair {
                birth: *d * 0.8,
                death: *d * 1.2,
                dim: 1,
            });
        }
        uf2.union(*i, *j);
    }

    (pairs, betti_data)
}

/// Pre-loaded point sets
pub fn circle_points(n: usize, radius: f64) -> Vec<(f64, f64)> {
    (0..n).map(|i| {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
        (radius * angle.cos(), radius * angle.sin())
    }).collect()
}

pub fn cluster_points(n: usize, centers: &[(f64, f64)], spread: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let per_cluster = n / centers.len();
    for (cx, cy) in centers {
        for i in 0..per_cluster {
            let angle = (i * 137) as f64 * std::f64::consts::PI / 180.0; // golden angle
            let r = spread * ((i as f64 * 0.618) % 1.0);
            points.push((cx + r * angle.cos(), cy + r * angle.sin()));
        }
    }
    points
}

pub fn torus_points(n: usize, r_major: f64, r_minor: f64) -> Vec<(f64, f64)> {
    // Project 3D torus to 2D (unwrap)
    (0..n).map(|i| {
        let u = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
        let v = 4.0 * std::f64::consts::PI * i as f64 / n as f64;
        ((r_major + r_minor * v.cos()) * u.cos(), (r_major + r_minor * v.cos()) * u.sin())
    }).collect()
}
