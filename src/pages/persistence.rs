use leptos::*;
use crate::components::persistence_diagram::PersistenceDiagram;
use crate::components::betti_curve::BettiCurve;
use crate::components::simplicial_complex::SimplicialComplex;
use crate::math::persistence::*;
use crate::math::simplex::build_rips;

#[component]
pub fn PersistencePage() -> impl IntoView {
    let (points, set_points) = create_signal(Vec::<(f64, f64)>::new());
    let (epsilon, set_epsilon) = create_signal(0.5);
    let (preset, set_preset) = create_signal(String::new());

    // Canvas click handler coordinates
    let (click_info, set_click_info) = create_signal(String::new());

    let canvas_ref = create_node_ref::<html::Canvas>();

    // Load preset
    let load_preset = move |name: &str| {
        let pts = match name {
            "circle" => circle_points(12, 0.4),
            "clusters" => cluster_points(15, &[(0.2, 0.3), (0.7, 0.3), (0.5, 0.8)], 0.1),
            "torus" => torus_points(16, 0.3, 0.15),
            _ => return,
        };
        set_points.set(pts);
        set_preset.set(name.to_string());
    };

    // Computed values
    let persistence_data = create_memo(move |_| {
        let pts = points.get();
        if pts.len() < 2 { return (vec![], vec![]) };
        compute_persistence(&pts, 1.5, 50)
    });

    let complex_data = create_memo(move |_| {
        let pts = points.get();
        let eps = epsilon.get();
        if pts.is_empty() { return (vec![], vec![], vec![]) }
        let complex = build_rips(&pts, eps);
        let verts: Vec<(f64, f64)> = pts.iter().map(|(x, y)| (*x, *y)).collect();
        (verts, complex.edges(), complex.triangles())
    });

    view! {
        <div class="page persistence-page">
            <h1>"Persistence Lab"</h1>
            <p class="page-desc">"Draw points on the canvas or load a preset. Watch the Vietoris-Rips filtration build."</p>

            <div class="controls">
                <div class="control-group">
                    <button on:click=move |_| load_preset("circle")>"⭕ Circle"</button>
                    <button on:click=move |_| load_preset("clusters")>"🔵 Clusters"</button>
                    <button on:click=move |_| load_preset("torus")>"🍩 Torus"</button>
                    <button on:click=move |_| { set_points.set(vec![]); set_preset.set(String::new()); }>"🗑 Clear"</button>
                </div>
                <div class="control-group">
                    <label>"ε = " {move || format!("{:.2}", epsilon.get())}</label>
                    <input
                        type="range"
                        min="0.05"
                        max="1.5"
                        step="0.01"
                        prop:value=move || epsilon.get()
                        on:input=move |e| set_epsilon.set(event_target_value(&e).parse().unwrap_or(0.5))
                    />
                </div>
                <div class="control-group">
                    <span>"Points: " {move || points.get().len()}</span>
                </div>
            </div>

            <div class="lab-grid">
                <div class="panel">
                    <h3>"Point Cloud"</h3>
                    <p class="hint">"Click to add points"</p>
                    <canvas
                        _ref=canvas_ref
                        width=500
                        height=400
                        class="point-canvas"
                        on:click=move |e| {
                            let canvas = canvas_ref.get().unwrap();
                            let rect = canvas.get_bounding_client_rect();
                            let x = (e.client_x() as f64 - rect.left()) / rect.width();
                            let y = (e.client_y() as f64 - rect.top()) / rect.height();
                            set_points.update(|pts| pts.push((x, y)));
                        }
                    />
                </div>

                <div class="panel">
                    <h3>"Simplicial Complex"</h3>
                    {move || {
                        let (v, e, t) = complex_data.get();
                        view! { <SimplicialComplex vertices=v edges=e triangles=t width=500.0 height=400.0/> }
                    }}
                </div>

                <div class="panel">
                    <h3>"Persistence Diagram"</h3>
                    {move || {
                        let (pairs, _) = persistence_data.get();
                        let pd_points: Vec<(f64, f64)> = pairs.iter()
                            .map(|p| (p.birth, p.death))
                            .collect();
                        view! { <PersistenceDiagram points=pd_points max_val=2.0 width=500.0 height=400.0/> }
                    }}
                </div>

                <div class="panel">
                    <h3>"Betti Curves"</h3>
                    {move || {
                        let (_, betti) = persistence_data.get();
                        view! { <BettiCurve data=betti width=500.0 height=400.0/> }
                    }}
                </div>
            </div>
        </div>
    }
}
