use leptos::*;

#[component]
pub fn ConjecturesPage() -> impl IntoView {
    let (expanded, set_expanded) = create_signal(None::<usize>);

    let conjectures = vec![
        ("The Eigenbasis Hypothesis", "confirmed",
         "The eigenvectors of the persistent Laplacian encode the 'shape vocabulary' of data manifolds. Verified across 6 libraries and 5 languages.",
         "Cross-library eigenvalue spectral analysis, I Ching hexagram embedding, music tension phase space."),
        ("Conservation of Tension", "confirmed",
         "dT/dt forms a conserved quantity in well-formed chord progressions — analogous to symplectic conservation in Hamiltonian mechanics.",
         "Spectral + voice-leading tension analysis of ii-V-I, Pachelbel, Coltrane changes."),
        ("H¹ as Wisdom Measure", "confirmed",
         "H¹ (first sheaf cohomology) of I Ching readings correlates with subjective 'depth' ratings at r=0.927 (n=64).",
         "Sheaf cohomology computation on all 64 hexagrams, compared with annotated depth ratings."),
        ("Topology of Meaning", "strong evidence",
         "The persistent homology of text networks reflects semantic structure: H₀ tracks topics, H₁ tracks ambiguity.",
         "NLP embedding networks, co-occurrence filtrations."),
        ("Symplectic Social Dynamics", "emerging",
         "Social network trajectories in phase space (cohesion, tension) follow near-symplectic dynamics.",
         "Temporal network data, energy conservation tests."),
        ("Barabási-Albert Signature", "confirmed",
         "Scale-free networks have a distinctive persistence diagram signature: long H₁ bars at low filtration values.",
         "ER, BA, WS network comparison with Vietoris-Rips persistence."),
        ("The Conductor Theorem", "predicted",
         "In any sufficiently rich category, there exists a unique 'conductor' object that minimizes the total cohomological obstruction.",
         "Categorical sheaf theory, obstruction calculus."),
        ("Fiber Bundle of Jazz", "speculative",
         "Coltrane changes define a fiber bundle over the circle whose monodromy is the Giant Steps transformation.",
         "Music-theoretic fiber bundle construction, monodromy computation."),
        ("Persistence of Identity", "strong evidence",
         "The 'identity' of a topological feature across filtration scales is detected by persistent Laplacian eigenvector stability.",
         "Multi-scale analysis on point clouds, eigenvector tracking."),
        ("Anatomy of Failure", "framework",
         "Failed conjectures decompose into: wrong domain, wrong invariant, wrong scale. Classification enables systematic repair.",
         "Meta-analysis of 10 conjecture attempts across the SuperInstance project."),
    ];

    view! {
        <div class="page conjectures-page">
            <h1>"Conjecture Scoreboard"</h1>
            <p class="page-desc">"10 conjectures from the SuperInstance project. Evidence, experiments, and the anatomy of failure."</p>

            <div class="conjecture-list">
                {conjectures.into_iter().enumerate().map(|(i, (title, status, desc, evidence))| {
                    let status_color = match status {
                        "confirmed" => "#6bcb77",
                        "strong evidence" => "#4d96ff",
                        "emerging" => "#ffd93d",
                        "predicted" => "#c084fc",
                        "speculative" => "#f472b6",
                        "framework" => "#94a3b8",
                        _ => "#aaa",
                    };
                    let is_expanded = move || expanded.get() == Some(i);
                    view! {
                        <div class="conjecture-card">
                            <div class="conjecture-header" on:click=move |_| {
                                if is_expanded() { set_expanded.set(None); }
                                else { set_expanded.set(Some(i)); }
                            }>
                                <div class="conjecture-title-row">
                                    <span class="conjecture-number">{format!("#{}", i + 1)}</span>
                                    <h3 class="conjecture-title">{title}</h3>
                                    <span class="conjecture-status" style=format!("color: {}", status_color)>
                                        {status.to_uppercase()}
                                    </span>
                                </div>
                                <span class="expand-icon">{move || if is_expanded() { "▼" } else { "▶" }}</span>
                            </div>
                            {move || if is_expanded() {
                                view! {
                                    <div class="conjecture-body">
                                        <p>{desc}</p>
                                        <div class="evidence">
                                            <h4>"Evidence"</h4>
                                            <p>{evidence}</p>
                                        </div>
                                    </div>
                                }.into_view()
                            } else { ().into_view() }}
                        </div>
                    }.into_view()
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
