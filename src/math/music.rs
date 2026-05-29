/// Music tension computation

/// Note in MIDI-like representation
#[derive(Clone, Copy, Debug)]
pub struct Note {
    pub pitch: f64, // MIDI note number (can be fractional for microtones)
    pub duration: f64,
}

/// A chord as a set of pitch classes
#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    pub name: String,
    pub pitches: Vec<f64>,
}

impl Chord {
    /// Spectral tension: how far from a consonant sonority
    pub fn spectral_tension(&self) -> f64 {
        if self.pitches.len() < 2 { return 0.0; }
        let mut tension = 0.0;
        for i in 0..self.pitches.len() {
            for j in (i + 1)..self.pitches.len() {
                let interval = (self.pitches[j] - self.pitches[i]).abs() % 12.0;
                // Tension based on interval quality (in semitones)
                tension += match interval as i32 % 12 {
                    0 | 7 => 0.0,          // unison, perfect fifth
                    5 => 0.1,              // perfect fourth
                    4 | 3 => 0.2,          // major/minor third
                    8 | 9 => 0.3,          // minor/major sixth
                    2 | 10 => 0.5,         // major/minor seventh
                    1 | 11 => 0.7,         // minor/major second
                    6 => 0.8,              // tritone
                    _ => 0.5,
                };
            }
        }
        // Normalize
        let pairs = (self.pitches.len() * (self.pitches.len() - 1)) as f64 / 2.0;
        tension / pairs
    }

    /// Voice-leading distance to another chord
    pub fn voice_leading_distance(&self, other: &Chord) -> f64 {
        if self.pitches.is_empty() || other.pitches.is_empty() { return 0.0; }
        let mut total = 0.0;
        let n = self.pitches.len().min(other.pitches.len());
        for i in 0..n {
            total += (self.pitches[i] - other.pitches[i]).abs();
        }
        total / n as f64
    }
}

/// Named chord progressions
pub fn ii_v_i() -> Vec<Chord> {
    vec![
        Chord { name: "Dm7".into(), pitches: vec![62.0, 65.0, 69.0, 72.0] },
        Chord { name: "G7".into(), pitches: vec![67.0, 70.0, 74.0, 77.0] },
        Chord { name: "Cmaj7".into(), pitches: vec![60.0, 64.0, 67.0, 72.0] },
    ]
}

pub fn pachelbel() -> Vec<Chord> {
    vec![
        Chord { name: "D".into(), pitches: vec![62.0, 66.0, 69.0] },
        Chord { name: "A".into(), pitches: vec![57.0, 61.0, 64.0] },
        Chord { name: "Bm".into(), pitches: vec![59.0, 62.0, 66.0] },
        Chord { name: "F#m".into(), pitches: vec![54.0, 57.0, 61.0] },
        Chord { name: "G".into(), pitches: vec![55.0, 59.0, 62.0] },
        Chord { name: "D".into(), pitches: vec![62.0, 66.0, 69.0] },
        Chord { name: "G".into(), pitches: vec![55.0, 59.0, 62.0] },
        Chord { name: "A".into(), pitches: vec![57.0, 61.0, 64.0] },
    ]
}

pub fn tristan() -> Vec<Chord> {
    vec![
        Chord { name: "Tristan¹".into(), pitches: vec![65.0, 68.0, 71.0, 74.0] },
        Chord { name: "Tristan²".into(), pitches: vec![64.0, 67.0, 70.0, 73.0] },
        Chord { name: "Tristan³".into(), pitches: vec![63.0, 66.0, 69.0, 72.0] },
    ]
}

pub fn coltrane() -> Vec<Chord> {
    vec![
        Chord { name: "Fmaj7".into(), pitches: vec![65.0, 69.0, 72.0, 76.0] },
        Chord { name: "E7".into(), pitches: vec![64.0, 68.0, 71.0, 74.0] },
        Chord { name: "Am7".into(), pitches: vec![57.0, 60.0, 64.0, 69.0] },
        Chord { name: "D7".into(), pitches: vec![62.0, 66.0, 69.0, 72.0] },
        Chord { name: "Gmaj7".into(), pitches: vec![55.0, 59.0, 62.0, 67.0] },
        Chord { name: "C#7".into(), pitches: vec![61.0, 65.0, 68.0, 71.0] },
        Chord { name: "F#m7".into(), pitches: vec![54.0, 57.0, 61.0, 66.0] },
        Chord { name: "B7".into(), pitches: vec![59.0, 63.0, 66.0, 69.0] },
        Chord { name: "Emaj7".into(), pitches: vec![52.0, 56.0, 59.0, 64.0] },
    ]
}

/// Compute tension curve for a chord progression
pub fn tension_curve(progression: &[Chord], resolution: usize) -> Vec<(f64, f64)> {
    let mut data = Vec::new();
    let total_time = progression.len() as f64;
    let dt = total_time / resolution as f64;

    for i in 0..resolution {
        let t = i as f64 * dt;
        let chord_idx = (t as usize).min(progression.len() - 1);
        let next_idx = (chord_idx + 1).min(progression.len() - 1);

        let spectral = progression[chord_idx].spectral_tension();
        let voice_lead = if chord_idx < progression.len() - 1 {
            progression[chord_idx].voice_leading_distance(&progression[next_idx])
        } else {
            0.0
        };

        let tension = 0.6 * spectral + 0.4 * (voice_lead / 6.0);
        data.push((t, tension));
    }

    data
}

/// Conservation score for a progression
pub fn conservation_score(progression: &[Chord]) -> f64 {
    if progression.len() < 2 { return 1.0; }
    let tensions: Vec<f64> = progression.iter().map(|c| c.spectral_tension()).collect();
    let mean = tensions.iter().sum::<f64>() / tensions.len() as f64;
    let variance = tensions.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / tensions.len() as f64;
    1.0 - variance.min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_tension_empty() {
        let chord = Chord { name: "empty".into(), pitches: vec![] };
        assert_eq!(chord.spectral_tension(), 0.0);
    }

    #[test]
    fn test_spectral_tension_single() {
        let chord = Chord { name: "single".into(), pitches: vec![60.0] };
        assert_eq!(chord.spectral_tension(), 0.0);
    }

    #[test]
    fn test_spectral_tension_octave_low() {
        let chord = Chord { name: "octave".into(), pitches: vec![60.0, 72.0] };
        assert_eq!(chord.spectral_tension(), 0.0); // unison interval
    }

    #[test]
    fn test_spectral_tension_tritone_high() {
        let tritone = Chord { name: "tritone".into(), pitches: vec![60.0, 66.0] };
        let fifth = Chord { name: "fifth".into(), pitches: vec![60.0, 67.0] };
        assert!(tritone.spectral_tension() > fifth.spectral_tension());
    }

    #[test]
    fn test_voice_leading_distance() {
        let c1 = Chord { name: "C".into(), pitches: vec![60.0, 64.0, 67.0] };
        let c2 = Chord { name: "C'".into(), pitches: vec![60.0, 64.0, 67.0] };
        assert_eq!(c1.voice_leading_distance(&c2), 0.0);
    }

    #[test]
    fn test_voice_leading_positive() {
        let c1 = Chord { name: "C".into(), pitches: vec![60.0, 64.0] };
        let c2 = Chord { name: "D".into(), pitches: vec![62.0, 66.0] };
        assert!(c1.voice_leading_distance(&c2) > 0.0);
    }

    #[test]
    fn test_ii_v_i_has_three_chords() {
        let prog = ii_v_i();
        assert_eq!(prog.len(), 3);
    }

    #[test]
    fn test_pachelbel_has_eight_chords() {
        let prog = pachelbel();
        assert_eq!(prog.len(), 8);
    }

    #[test]
    fn test_tension_curve_length() {
        let prog = ii_v_i();
        let curve = tension_curve(&prog, 100);
        assert_eq!(curve.len(), 100);
    }

    #[test]
    fn test_conservation_score_range() {
        let prog = pachelbel();
        let score = conservation_score(&prog);
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_conservation_single_chord() {
        let prog = vec![Chord { name: "C".into(), pitches: vec![60.0, 64.0, 67.0] }];
        assert_eq!(conservation_score(&prog), 1.0);
    }
}
