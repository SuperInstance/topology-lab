use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="page home-page">
            <section class="hero">
                <h1 class="hero-title">
                    <span class="hero-icon">{"\u{222c}"}</span>
                    " Topology Lab"
                </h1>
                <p class="hero-subtitle">"Interactive Mathematical Visualization \u{2014} running entirely in your browser"</p>
                <div class="hero-badges">
                    <span class="badge rust">"\u{1f980} Pure Rust"</span>
                    <span class="badge wasm">"\u{26a1} WASM"</span>
                    <span class="badge math">"\u{1f9ee} Client-Side Math"</span>
                </div>
            </section>

            <section class="features">
                <h2>"Labs"</h2>
                <div class="feature-grid">
                    <a href="/persistence" class="feature-card">
                        <div class="feature-icon">"\u{1f535}"</div>
                        <h3>"Persistence Lab"</h3>
                        <p>"Draw points, compute Vietoris-Rips filtration, watch persistence diagrams appear in real-time"</p>
                    </a>
                    <a href="/symplectic" class="feature-card">
                        <div class="feature-icon">"\u{1f300}"</div>
                        <h3>"Symplectic Playground"</h3>
                        <p>"Compare Euler, Verlet, and Yoshida integrators. Watch energy conservation or the lack of it."</p>
                    </a>
                    <a href="/iching" class="feature-card">
                        <div class="feature-icon">"\u{262f}"</div>
                        <h3>"I Ching Oracle"</h3>
                        <p>"Cast a reading with live sheaf cohomology. H0 measures agreement. H1 measures wisdom."</p>
                    </a>
                    <a href="/music" class="feature-card">
                        <div class="feature-icon">"\u{266b}"</div>
                        <h3>"Music Explorer"</h3>
                        <p>"Visualize tension curves of chord progressions. The tension gradient dT/dt is our confirmed prediction."</p>
                    </a>
                    <a href="/social" class="feature-card">
                        <div class="feature-icon">"\u{1f578}"</div>
                        <h3>"Social Networks"</h3>
                        <p>"Generate random networks and compute their topological fingerprints."</p>
                    </a>
                    <a href="/conjectures" class="feature-card">
                        <div class="feature-icon">"\u{1f4cb}"</div>
                        <h3>"Conjecture Board"</h3>
                        <p>"10 conjectures. Experimental evidence. The Eigenbasis Hypothesis."</p>
                    </a>
                </div>
            </section>

            <section class="stats">
                <h2>"SuperInstance Ecosystem"</h2>
                <div class="stat-row">
                    <div class="stat">
                        <div class="stat-number">"8"</div>
                        <div class="stat-label">"Libraries"</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">"6"</div>
                        <div class="stat-label">"Languages"</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">"10"</div>
                        <div class="stat-label">"Conjectures"</div>
                    </div>
                    <div class="stat">
                        <div class="stat-number">"\u{221e}"</div>
                        <div class="stat-label">"Topological Curiosity"</div>
                    </div>
                </div>
            </section>
        </div>
    }
}
