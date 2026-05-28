use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use leptos::*;

/// Symplectic phase space (q, p) trajectory viewer
#[component]
pub fn PhaseSpace(
    trajectory: Vec<(f64, f64)>,
    width: f64,
    height: f64,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    create_effect(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();

        let pad = 40.0;
        let plot_w = width - 2.0 * pad;
        let plot_h = height - 2.0 * pad;

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.set_fill_style(&JsValue::from_str("#1a1a2e"));
        ctx.fill_rect(0.0, 0.0, width, height);

        if trajectory.len() < 2 { return; }

        let q_min = trajectory.iter().map(|(q, _)| *q).fold(f64::INFINITY, f64::min);
        let q_max = trajectory.iter().map(|(q, _)| *q).fold(f64::NEG_INFINITY, f64::max);
        let p_min = trajectory.iter().map(|(_, p)| *p).fold(f64::INFINITY, f64::min);
        let p_max = trajectory.iter().map(|(_, p)| *p).fold(f64::NEG_INFINITY, f64::max);

        let q_range = (q_max - q_min).max(0.1);
        let p_range = (p_max - p_min).max(0.1);
        let margin = 0.1;

        // Draw trajectory with gradient
        for i in 1..trajectory.len() {
            let (q0, p0) = trajectory[i - 1];
            let (q1, p1) = trajectory[i];

            let x0 = pad + ((q0 - q_min) / q_range) * plot_w;
            let y0 = pad + plot_h - ((p0 - p_min) / p_range) * plot_h;
            let x1 = pad + ((q1 - q_min) / q_range) * plot_w;
            let y1 = pad + plot_h - ((p1 - p_min) / p_range) * plot_h;

            let t = i as f64 / trajectory.len() as f64;
            let r = (107.0 + 148.0 * t) as u8;
            let g = (107.0 + 94.0 * (1.0 - t)) as u8;
            let b = (107.0 + 148.0 * t) as u8;
            ctx.set_stroke_style(&JsValue::from_str(&format!("rgb({},{},{})", r, g, b)));
            ctx.set_line_width(1.5);
            ctx.begin_path();
            ctx.move_to(x0, y0);
            ctx.line_to(x1, y1);
            ctx.stroke();
        }

        // Start point
        if let Some((q, p)) = trajectory.first() {
            let x = pad + ((q - q_min) / q_range) * plot_w;
            let y = pad + plot_h - ((p - p_min) / p_range) * plot_h;
            ctx.set_fill_style(&JsValue::from_str("#6bcb77"));
            ctx.begin_path();
            ctx.arc(x, y, 6.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.fill();
        }

        // Labels
        ctx.set_fill_style(&JsValue::from_str("#aaaacc"));
        ctx.set_font("12px monospace");
        ctx.fill_text("q (position)", pad + plot_w / 2.0 - 30.0, height - 8.0);
        ctx.save();
        ctx.translate(12.0, pad + plot_h / 2.0).unwrap();
        ctx.rotate(-std::f64::consts::FRAC_PI_2).unwrap();
        ctx.fill_text("p (momentum)", -35.0, 0.0);
        ctx.restore();
    });

    view! {
        <canvas _ref=canvas_ref width=width as u32 height=height as u32 class="phase-canvas"/>
    }
}
