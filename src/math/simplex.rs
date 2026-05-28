/// Pure Rust simplex/complex types for topological computation

#[derive(Clone, Debug, PartialEq)]
pub struct Simplex {
    pub vertices: Vec<usize>,
}

impl Simplex {
    pub fn new(vertices: Vec<usize>) -> Self {
        let mut v = vertices;
        v.sort();
        v.dedup();
        Self { vertices: v }
    }

    pub fn dimension(&self) -> usize {
        if self.vertices.is_empty() { return 0; }
        self.vertices.len() - 1
    }

    pub fn faces(&self) -> Vec<Simplex> {
        if self.vertices.len() <= 1 { return vec![]; }
        let mut faces = Vec::new();
        for i in 0..self.vertices.len() {
            let mut face = self.vertices.clone();
            face.remove(i);
            faces.push(Simplex::new(face));
        }
        faces
    }

    pub fn contains(&self, other: &Simplex) -> bool {
        other.vertices.iter().all(|v| self.vertices.contains(v))
    }
}

#[derive(Clone, Debug, Default)]
pub struct SimplicialComplex {
    pub simplices: Vec<Simplex>,
}

impl SimplicialComplex {
    pub fn new() -> Self {
        Self { simplices: Vec::new() }
    }

    pub fn add_simplex(&mut self, simplex: Simplex) {
        if !self.simplices.contains(&simplex) {
            // Add all faces too
            for face in simplex.faces() {
                self.add_simplex(face);
            }
            self.simplices.push(simplex);
        }
    }

    pub fn vertices(&self) -> Vec<usize> {
        let mut verts: Vec<usize> = self.simplices.iter()
            .flat_map(|s| s.vertices.iter().copied())
            .collect();
        verts.sort();
        verts.dedup();
        verts
    }

    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.simplices.iter()
            .filter(|s| s.vertices.len() == 2)
            .map(|s| (s.vertices[0], s.vertices[1]))
            .collect()
    }

    pub fn triangles(&self) -> Vec<(usize, usize, usize)> {
        self.simplices.iter()
            .filter(|s| s.vertices.len() == 3)
            .map(|s| (s.vertices[0], s.vertices[1], s.vertices[2]))
            .collect()
    }
}

/// Build Vietoris-Rips complex from points at given epsilon
pub fn build_rips(points: &[(f64, f64)], epsilon: f64) -> SimplicialComplex {
    let n = points.len();
    let mut complex = SimplicialComplex::new();

    // Add vertices
    for i in 0..n {
        complex.add_simplex(Simplex::new(vec![i]));
    }

    // Add edges within epsilon
    let mut adj = vec![vec![false; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= epsilon {
                complex.add_simplex(Simplex::new(vec![i, j]));
                adj[i][j] = true;
                adj[j][i] = true;
            }
        }
    }

    // Add triangles where all three edges exist
    for i in 0..n {
        for j in (i + 1)..n {
            if !adj[i][j] { continue; }
            for k in (j + 1)..n {
                if adj[i][k] && adj[j][k] {
                    complex.add_simplex(Simplex::new(vec![i, j, k]));
                }
            }
        }
    }

    complex
}

/// Union-Find for computing H0
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            match self.rank[rx].cmp(&self.rank[ry]) {
                std::cmp::Ordering::Less => self.parent[rx] = ry,
                std::cmp::Ordering::Greater => self.parent[ry] = rx,
                std::cmp::Ordering::Equal => {
                    self.parent[ry] = rx;
                    self.rank[rx] += 1;
                }
            }
        }
    }

    pub fn count_components(&mut self, n: usize) -> usize {
        let mut roots: Vec<usize> = (0..n).map(|i| self.find(i)).collect();
        roots.sort();
        roots.dedup();
        roots.len()
    }
}
