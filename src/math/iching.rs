/// I Ching hexagram logic

/// All 64 hexagrams: (number, name, lines [6 bools: true=yang, false=yin], judgment excerpt)
pub const HEXAGRAMS: [(u8, &str, [bool; 6], &str); 64] = [
    (1, "The Creative", [true,true,true,true,true,true], "Sublime success. Perseverance furthers."),
    (2, "The Receptive", [false,false,false,false,false,false], "Sublime success. Devotion bears fruit."),
    (3, "Difficulty at Beginning", [true,false,false,false,true,false], "Success through perseverance."),
    (4, "Youthful Folly", [false,true,false,false,false,true], "Not I seek the youth, the youth seeks me."),
    (5, "Waiting", [true,false,true,true,false,false], "Sincerity brings brilliant success."),
    (6, "Conflict", [false,false,true,true,false,true], "If right, energetic contention furthers."),
    (7, "The Army", [false,false,false,true,false,false], "The army needs perseverance and a strong leader."),
    (8, "Holding Together", [false,true,false,false,false,false], "Success. Hunt in the south."),
    (9, "Small Taming", [true,true,false,true,false,false], "Dense clouds but no rain."),
    (10, "Treading", [false,false,true,true,true,false], "Treading upon the tail of the tiger."),
    (11, "Peace", [false,false,false,true,true,true], "The small departs, the great approaches."),
    (12, "Standstill", [true,true,true,false,false,false], "The great departs, the small approaches."),
    (13, "Fellowship", [true,false,true,true,true,false], "Success in the open. Perseverance furthers."),
    (14, "Great Possession", [true,true,true,true,false,true], "Supreme success."),
    (15, "Modesty", [false,false,false,true,false,false], "The superior one reduces the great and increases the small."),
    (16, "Enthusiasm", [false,false,false,false,true,true], "Firm as thunder. The host assembles."),
    (17, "Following", [true,false,false,true,true,false], "Following has supreme success. Perseverance furthers."),
    (18, "Work on Decay", [false,true,true,false,false,true], "Supreme success. Before crossing the great water."),
    (19, "Approach", [false,false,true,true,true,true], "The eighth month brings misfortune."),
    (20, "Contemplation", [true,true,true,true,false,false], "Contemplate the divine way of heaven."),
    (21, "Biting Through", [true,false,false,true,false,true], "Success. Legal proceedings justified."),
    (22, "Grace", [true,false,false,true,false,false], "Success in small matters."),
    (23, "Splitting Apart", [false,false,false,false,false,true], "Not favorable for going anywhere."),
    (24, "Return", [true,false,false,false,false,false], "Success. Going out and coming in without error."),
    (25, "Innocence", [true,false,false,true,false,false], "Supreme success. Favorable to undertake something."),
    (26, "Taming Power of Great", [true,true,true,false,false,true], "Perseverance furthers. Not eating at home."),
    (27, "Nourishment", [true,false,false,false,false,true], "Perseverance brings good fortune."),
    (28, "Preponderance of Great", [false,false,true,true,false,false], "The ridgepole sags to the breaking point."),
    (29, "Abysmal Water", [false,true,false,false,true,false], "Repetition of danger. Sincerity."),
    (30, "Clinging Fire", [true,false,true,true,false,true], "Favorable through perseverance. Care of the cow."),
    (31, "Influence", [false,false,true,true,false,false], "Taking a maiden brings good fortune."),
    (32, "Duration", [false,false,true,true,false,false], "Success. No blame. Perseverance furthers."),
    (33, "Retreat", [false,false,true,true,true,true], "Success. The small serves the great."),
    (34, "Power of Great", [true,true,true,true,false,false], "Righteous perseverance furthers."),
    (35, "Progress", [false,false,false,true,true,false], "A powerful prince honored with horses."),
    (36, "Darkening of Light", [true,false,true,false,false,false], "Darkness. Advantageous to be persevering."),
    (37, "Family", [true,false,true,false,true,false], "The family's fate depends on the woman."),
    (38, "Opposition", [true,false,true,true,false,true], "In small matters, good fortune."),
    (39, "Obstruction", [false,false,true,false,true,false], "The southwest furthers. The northeast does not."),
    (40, "Deliverance", [false,true,false,true,false,false], "The southwest furthers. Return brings good fortune."),
    (41, "Decrease", [true,true,false,false,true,true], "Supreme success. No blame. Perseverance."),
    (42, "Increase", [true,true,true,true,false,false], "Favorable to undertake something. Crossing the great water."),
    (43, "Breakthrough", [true,true,true,true,true,false], "One must resolutely make known the truth."),
    (44, "Coming to Meet", [false,true,true,true,true,true], "The maiden is powerful. Do not marry her."),
    (45, "Gathering Together", [false,false,false,true,true,false], "Success. The king approaches the temple."),
    (46, "Pushing Upward", [false,false,false,false,true,true], "Supreme success. The king offers sacrifice."),
    (47, "Oppression", [false,true,false,true,false,false], "Exhaustion yet perseverance. Words do not convince."),
    (48, "The Well", [false,false,true,false,true,false], "The well is cleaned but not used."),
    (49, "Revolution", [true,false,true,true,false,true], "On your own day, you are believed."),
    (50, "The Caldron", [true,true,false,true,false,true], "Supreme good fortune. Success."),
    (51, "Arousing Thunder", [true,false,false,true,false,false], "Shock brings success. Laughter and talking."),
    (52, "Keeping Still", [false,false,true,false,false,true], "Rest in the back. No body."),
    (53, "Development", [false,false,true,true,false,false], "The wild goose gradually approaches the shore."),
    (54, "Marrying Maiden", [true,false,true,false,true,false], "Undertakings bring misfortune."),
    (55, "Abundance", [true,false,true,true,false,true], "Success. The king approaches. Do not mourn."),
    (56, "The Wanderer", [false,false,true,true,false,true], "Success in small matters. Perseverance brings good fortune."),
    (57, "Gentle Wind", [false,true,true,false,true,true], "Success through small undertakings."),
    (58, "Joyous Lake", [true,true,false,true,true,false], "Success. Perseverance is favorable."),
    (59, "Dispersion", [false,true,false,false,true,false], "Success. The king approaches the temple."),
    (60, "Limitation", [true,false,false,true,true,false], "Galling limitation cannot be persevered in."),
    (61, "Inner Truth", [true,false,false,false,false,true], "Pigs and fishes. Good fortune."),
    (62, "Preponderance of Small", [false,false,true,false,true,true], "Flying bird brings message. Not suitable for great."),
    (63, "After Completion", [true,false,true,false,true,false], "Success in small matters. Fox gets wet tail."),
    (64, "Before Completion", [false,true,false,true,false,true], "Success. The fox gets his tail wet."),
];

/// Cast an I Ching reading using the three-coin method
pub fn cast_reading() -> (u8, [bool; 6], Vec<u8>) {
    let mut lines = [false; 6];
    let mut line_values = Vec::new();

    for i in 0..6 {
        // Three coins: heads=3, tails=2
        let coin1 = if pseudo_random() > 0.5 { 3 } else { 2 };
        let coin2 = if pseudo_random() > 0.5 { 3 } else { 2 };
        let coin3 = if pseudo_random() > 0.5 { 3 } else { 2 };
        let sum = coin1 + coin2 + coin3;
        lines[i] = sum % 2 == 1; // 6=yin(0), 7=yang(1), 8=yin(0), 9=yang(1)
        line_values.push(sum as u8);
    }

    // Find matching hexagram
    let number = find_hexagram(&lines);
    (number, lines, line_values)
}

fn pseudo_random() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    // Simple hash
    let x = (nanos.wrapping_mul(1103515245).wrapping_add(12345)) as f64;
    (x % 1000000.0) / 1000000.0
}

fn find_hexagram(lines: &[bool; 6]) -> u8 {
    for (num, _, hex_lines, _) in &HEXAGRAMS {
        if hex_lines == lines { return *num; }
    }
    // Approximate: find closest match
    let mut best = 1u8;
    let mut best_score = 0;
    for (num, _, hex_lines, _) in &HEXAGRAMS {
        let score = lines.iter().zip(hex_lines.iter()).filter(|(a, b)| a == b).count();
        if score > best_score {
            best_score = score;
            best = *num;
        }
    }
    best
}

/// Get hexagram data by number
pub fn get_hexagram(number: u8) -> Option<(u8, &'static str, [bool; 6], &'static str)> {
    HEXAGRAMS.iter().find(|(n, _, _, _)| *n == number).copied()
}
