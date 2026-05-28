use leptos::*;
use crate::components::tension_curve::TensionCurve;
use crate::components::phase_space::PhaseSpace;
use crate::math::music::*;

#[component]
pub fn MusicPage() -> impl IntoView {
    let (progression_idx, set_progression_idx) = create_signal(0u8);

    let progression_data = create_memo(move |_| {
        let progression: Vec<Chord> = match progression_idx.get() {
            0 => ii_v_i(),
            1 => pachelbel(),
            2 => tristan(),
            _ => coltrane(),
        };
        progression
    });

    let tension_data = create_memo(move |_| {
        let prog = progression_data.get();
        tension_curve(&prog, 200)
    });

    let phase_data = create_memo(move |_| {
        let t = tension_data.get();
        if t.len() < 2 { return vec![] };
        let mut phase = Vec::new();
        for w in t.windows(2) {
            let dt = w[1].0 - w[0].0;
            let dv = w[1].1 - w[0].1;
            let grad = if dt.abs() > 1e-10 { dv / dt } else { 0.0 };
            phase.push((w[0].1, grad));
        }
        phase
    });

    let conservation = create_memo(move |_| {
        conservation_score(&progression_data.get())
    });

    view! {
        <div class="page music-page">
            <h1>"Music Explorer"</h1>
            <p class="page-desc">"Visualize tension curves of chord progressions. The tension gradient dT/dt is our confirmed prediction."</p>

            <div class="controls">
                <div class="control-group">
                    <label>"Progression:"</label>
                    <select on:change=move |e| set_progression_idx.set(event_target_value(&e).parse().unwrap_or(0))>
                        <option value="0">"ii-V-I (Jazz)"</option>
                        <option value="1">"Pachelbel Canon"</option>
                        <option value="2">"Tristan Chord"</option>
                        <option value="3">"Coltrane Changes"</option>
                    </select>
                </div>
            </div>

            <div class="chord-display">
                {move || {
                    let prog = progression_data.get();
                    prog.iter().map(|c| view! {
                        <span class="chord-badge">{c.name.clone()}</span>
                    }).collect::<Vec<_>>()
                }}
            </div>

            <div class="lab-grid">
                <div class="panel wide">
                    <h3>"Tension Curve"</h3>
                    {move || view! { <TensionCurve tension_data=tension_data.get() width=700.0 height=350.0/> }}
                </div>

                <div class="panel">
                    <h3>"Phase Space (T, dT/dt)"</h3>
                    {move || view! { <PhaseSpace trajectory=phase_data.get() width=500.0 height=350.0/> }}
                </div>

                <div class="panel">
                    <h3>"Conservation Score"</h3>
                    <div class="conservation-score">
                        <div class="score-ring">
                            <svg viewBox="0 0 100 100">
                                <circle cx="50" cy="50" r="45" fill="none" stroke="#2a2a4e" stroke-width="6"/>
                                <circle
                                    cx="50" cy="50" r="45" fill="none"
                                    stroke="#6bcb77"
                                    stroke-width="6"
                                    stroke-dasharray=format!("{} {}", conservation.get() * 282.7, 282.7)
                                    transform="rotate(-90 50 50)"
                                />
                            </svg>
                            <div class="score-value">
                                {move || format!("{:.0}%", conservation.get() * 100.0)}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
