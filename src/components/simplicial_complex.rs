use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use leptos::*;

/// 2D simplicial complex visualization
#[component]
pub fn SimplicialComplex(
    vertices: Vec<(f64, f64)>,
    edges: Vec<(usize, usize)>,
    triangles: Vec<(usize, usize, usize)>,
    width: f64,
    height: f64,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    create_effect(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();

        let pad = 20.0;

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.set_fill_style(&JsValue::from_str("#0f0f1a"));
        ctx.fill_rect(0.0, 0.0, width, height);

        // Triangles
        ctx.set_fill_style(&"rgba(77, 150, 255, 0.15)".into());
        ctx.set_stroke_style(&"rgba(77, 150, 255, 0.3)".into());
        for (i, j, k) in &triangles {
            if let (Some(a), Some(b), Some(c)) = (vertices.get(*i), vertices.get(*j), vertices.get(*k)) {
                let ax = pad + a.0 * (width - 2.0 * pad);
                let ay = pad + a.1 * (height - 2.0 * pad);
                let bx = pad + b.0 * (width - 2.0 * pad);
                let by = pad + b.1 * (height - 2.0 * pad);
                let cx = pad + c.0 * (width - 2.0 * pad);
                let cy = pad + c.1 * (height - 2.0 * pad);
                ctx.begin_path();
                ctx.move_to(ax, ay);
                ctx.line_to(bx, by);
                ctx.line_to(cx, cy);
                ctx.close_path();
                ctx.fill();
                ctx.stroke();
            }
        }

        // Edges
        ctx.set_stroke_style(&JsValue::from_str("#4d96ff"));
        ctx.set_line_width(1.5);
        for (i, j) in &edges {
            if let (Some(a), Some(b)) = (vertices.get(*i), vertices.get(*j)) {
                let ax = pad + a.0 * (width - 2.0 * pad);
                let ay = pad + a.1 * (height - 2.0 * pad);
                let bx = pad + b.0 * (width - 2.0 * pad);
                let by = pad + b.1 * (height - 2.0 * pad);
                ctx.begin_path();
                ctx.move_to(ax, ay);
                ctx.line_to(bx, by);
                ctx.stroke();
            }
        }

        // Vertices
        for v in &vertices {
            let x = pad + v.0 * (width - 2.0 * pad);
            let y = pad + v.1 * (height - 2.0 * pad);
            ctx.set_fill_style(&JsValue::from_str("#ff6b6b"));
            ctx.begin_path();
            ctx.arc(x, y, 5.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.fill();
        }
    });

    view! {
        <canvas _ref=canvas_ref width=width as u32 height=height as u32 class="complex-canvas"/>
    }
}
