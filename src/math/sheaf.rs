/// Sheaf cohomology computation

/// A simple sheaf on a poset (represented as adjacency)
pub struct SimpleSheaf {
    /// Number of local sections (one per open set)
    pub sections: Vec<f64>,
    /// Restriction maps: (from_idx, to_idx, weight)
    pub restrictions: Vec<(usize, usize, f64)>,
}

impl SimpleSheaf {
    /// Compute H0: global sections (agreement)
    pub fn h0(&self) -> f64 {
        if self.sections.is_empty() { return 0.0; }
        let n = self.sections.len();
        let mut agreement = 0.0;
        let mut count = 0;

        for (i, j, w) in &self.restrictions {
            if *i < n && *j < n {
                let diff = (self.sections[*i] - self.sections[*j]).abs();
                agreement += (1.0 - diff) * w;
                count += 1;
            }
        }

        if count > 0 {
            let base = self.sections.iter().sum::<f64>() / n as f64;
            (agreement / count as f64 * base).abs().min(1.0)
        } else {
            1.0
        }
    }

    /// Compute H1: obstruction (information lost in gluing)
    pub fn h1(&self) -> f64 {
        if self.sections.is_empty() { return 0.0; }
        let n = self.sections.len();
        let mut obstruction = 0.0;
        let mut count = 0;

        for (i, j, w) in &self.restrictions {
            if *i < n && *j < n {
                let diff = (self.sections[*i] - self.sections[*j]).abs();
                obstruction += diff * w;
                count += 1;
            }
        }

        if count > 0 {
            (obstruction / count as f64).abs().min(1.0)
        } else {
            0.0
        }
    }
}

/// Build sheaf from I Ching hexagram lines
pub fn hexagram_sheaf(lines: &[bool; 6]) -> SimpleSheaf {
    // Each line is an open set with a section value
    // Yang (solid) = 1.0, Yin (broken) = 0.5
    let sections: Vec<f64> = lines.iter().map(|&yang| if yang { 1.0 } else { 0.5 }).collect();

    // Restriction maps between adjacent lines (trigram structure)
    let mut restrictions = Vec::new();
    for i in 0..5 {
        let weight = if i < 2 || i >= 3 { 0.8 } else { 0.6 }; // within trigrams stronger
        restrictions.push((i, i + 1, weight));
    }
    // Cross-trigram connections
    restrictions.push((2, 3, 0.4));

    SimpleSheaf { sections, restrictions }
}
