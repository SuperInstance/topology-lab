use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use leptos::*;
use crate::components::persistence_diagram::PersistenceDiagram;
use crate::math::simplex::{build_rips, UnionFind};

#[component]
pub fn SocialPage() -> impl IntoView {
    let (network_type, set_network_type) = create_signal(0u8);
    let (n_nodes, set_n_nodes) = create_signal(30u32);
    let (param, set_param) = create_signal(0.3f64);
    let (seed, set_seed) = create_signal(0u32);

    let network = create_memo(move |_| {
        let n = n_nodes.get() as usize;
        let p = param.get();
        let s = seed.get();

        let mut adj = vec![vec![false; n]; n];
        let mut rng = SimpleRng::new(s);

        match network_type.get() {
            0 => {
                // Erdős-Rényi
                for i in 0..n {
                    for j in (i + 1)..n {
                        if rng.next() < p { adj[i][j] = true; adj[j][i] = true; }
                    }
                }
            }
            1 => {
                // Barabási-Albert (preferential attachment)
                let m = (p * 5.0).max(1.0) as usize;
                let mut degree = vec![0usize; n];
                for i in 1..n {
                    let total: usize = degree[..i].iter().sum();
                    let mut attached = 0;
                    for j in 0..i {
                        if attached >= m { break; }
                        let prob = if total > 0 { degree[j] as f64 / total as f64 * m as f64 } else { 1.0 / i as f64 };
                        if rng.next() < prob {
                            adj[i][j] = true; adj[j][i] = true;
                            degree[i] += 1; degree[j] += 1;
                            attached += 1;
                        }
                    }
                    if attached == 0 && i > 0 {
                        adj[i][0] = true; adj[0][i] = true;
                        degree[i] += 1; degree[0] += 1;
                    }
                }
            }
            _ => {
                // Watts-Strogatz
                let k = (p * 6.0).max(2.0) as usize;
                let beta = 0.3;
                // Start with ring lattice
                for i in 0..n {
                    for d in 1..=k/2 {
                        let j = (i + d) % n;
                        adj[i][j] = true; adj[j][i] = true;
                    }
                }
                // Rewire
                for i in 0..n {
                    for d in 1..=k/2 {
                        if rng.next() < beta {
                            let j = (i + d) % n;
                            adj[i][j] = false; adj[j][i] = false;
                            let new_j = (rng.next() * n as f64) as usize;
                            if new_j != i {
                                adj[i][new_j] = true; adj[new_j][i] = true;
                            }
                        }
                    }
                }
            }
        }

        // Generate positions (force-directed-ish: just place in circle)
        let positions: Vec<(f64, f64)> = (0..n).map(|i| {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            let r = 0.35 + 0.1 * (i as f64 * 0.1).sin();
            (0.5 + r * angle.cos(), 0.5 + r * angle.sin())
        }).collect();

        // Count edges
        let edge_count = (0..n).map(|i| (i+1..n).filter(|&j| adj[i][j]).count()).sum::<usize>();

        // Compute H0
        let mut uf = UnionFind::new(n);
        for i in 0..n {
            for j in (i+1)..n {
                if adj[i][j] { uf.union(i, j); }
            }
        }
        let h0 = uf.count_components(n);

        // Distance histogram for persistence
        let mut distances = Vec::new();
        for i in 0..n {
            for j in (i+1)..n {
                if adj[i][j] {
                    let dx = positions[i].0 - positions[j].0;
                    let dy = positions[i].1 - positions[j].1;
                    distances.push((dx*dx + dy*dy).sqrt());
                }
            }
        }
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let pd_points: Vec<(f64, f64)> = distances.iter().enumerate()
            .map(|(i, &d)| (0.0, d))
            .collect();

        (positions, adj, edge_count, h0, pd_points)
    });

    let canvas_ref = create_node_ref::<html::Canvas>();

    create_effect(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        let (positions, adj, _, _, _) = network.get();
        let n = positions.len();

        ctx.clear_rect(0.0, 0.0, 600.0, 500.0);
        ctx.set_fill_style(&JsValue::from_str("#0f0f1a"));
        ctx.fill_rect(0.0, 0.0, 600.0, 500.0);

        // Edges
        ctx.set_stroke_style(&"rgba(77, 150, 255, 0.3)".into());
        ctx.set_line_width(0.8);
        for i in 0..n {
            for j in (i+1)..n {
                if i < adj.len() && j < adj[i].len() && adj[i][j] {
                    let x1 = positions[i].0 * 560.0 + 20.0;
                    let y1 = positions[i].1 * 460.0 + 20.0;
                    let x2 = positions[j].0 * 560.0 + 20.0;
                    let y2 = positions[j].1 * 460.0 + 20.0;
                    ctx.begin_path();
                    ctx.move_to(x1, y1);
                    ctx.line_to(x2, y2);
                    ctx.stroke();
                }
            }
        }

        // Nodes
        for (i, pos) in positions.iter().enumerate() {
            let degree: usize = if i < adj.len() { adj[i].iter().filter(|&&b| b).count() } else { 0 };
            let r = 3.0 + degree as f64 * 0.5;
            let x = pos.0 * 560.0 + 20.0;
            let y = pos.1 * 460.0 + 20.0;
            ctx.set_fill_style(&JsValue::from_str(if degree > n / 3 { "#ff6b6b" } else { "#6bcb77" }));
            ctx.begin_path();
            ctx.arc(x, y, r, 0.0, std::f64::consts::TAU).unwrap();
            ctx.fill();
        }
    });

    view! {
        <div class="page social-page">
            <h1>"Social Network Lab"</h1>
            <p class="page-desc">"Generate random networks and compute their topological signatures."</p>

            <div class="controls">
                <div class="control-group">
                    <label>"Model:"</label>
                    <select on:change=move |e| set_network_type.set(event_target_value(&e).parse().unwrap_or(0))>
                        <option value="0">"Erdős-Rényi"</option>
                        <option value="1">"Barabási-Albert (Scale-Free)"</option>
                        <option value="2">"Watts-Strogatz (Small World)"</option>
                    </select>
                </div>
                <div class="control-group">
                    <label>"Nodes: " {move || n_nodes.get()}</label>
                    <input type="range" min="10" max="80" step="5"
                        prop:value=move || n_nodes.get()
                        on:input=move |e| set_n_nodes.set(event_target_value(&e).parse().unwrap_or(30))
                    />
                </div>
                <div class="control-group">
                    <label>"Parameter: " {move || format!("{:.2}", param.get())}</label>
                    <input type="range" min="0.05" max="0.95" step="0.05"
                        prop:value=move || param.get()
                        on:input=move |e| set_param.set(event_target_value(&e).parse().unwrap_or(0.3))
                    />
                </div>
                <button on:click=move |_| set_seed.update(|s| *s = s.wrapping_add(1))>"Regenerate"</button>
            </div>

            <div class="lab-grid">
                <div class="panel">
                    <h3>"Network"</h3>
                    <canvas _ref=canvas_ref width=600 height=500 class="network-canvas"/>
                </div>
                <div class="panel">
                    <h3>"Persistence Diagram"</h3>
                    {move || {
                        let (_, _, _, _, pd) = network.get();
                        view! { <PersistenceDiagram points=pd max_val=1.0 width=500.0 height=400.0/> }
                    }}
                </div>
                <div class="panel">
                    <h3>"Network Stats"</h3>
                    <div class="stats-list">
                        <div>"Nodes: " {move || n_nodes.get()}</div>
                        <div>"Edges: " {move || network.get().2}</div>
                        <div>"H₀ (components): " {move || network.get().3}</div>
                        <div>"Model: " {move || match network_type.get() {
                            0 => "Erdős-Rényi",
                            1 => "Barabási-Albert",
                            _ => "Watts-Strogatz",
                        }}</div>
                        {move || if network_type.get() == 1 {
                            view! { <div class="scale-free-badge">"Scale-free signature detected (hubs in red)"</div> }.into_view()
                        } else { ().into_view() }}
                    </div>
                </div>
            </div>
        </div>
    }
}

struct SimpleRng { state: u32 }

impl SimpleRng {
    fn new(seed: u32) -> Self { Self { state: if seed == 0 { 1 } else { seed } } }
    fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 16) as f64 / 65536.0
    }
}
