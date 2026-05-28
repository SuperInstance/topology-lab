use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use leptos::*;

/// Betti curve: Betti numbers vs epsilon
#[component]
pub fn BettiCurve(
    data: Vec<(f64, Vec<usize>)>, // (epsilon, [H0, H1, H2])
    width: f64,
    height: f64,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    create_effect(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();

        let w = width;
        let h = height;
        let pad = 40.0;
        let plot_w = w - 2.0 * pad;
        let plot_h = h - 2.0 * pad;

        ctx.clear_rect(0.0, 0.0, w, h);
        ctx.set_fill_style(&JsValue::from_str("#1a1a2e"));
        ctx.fill_rect(0.0, 0.0, w, h);

        if data.is_empty() { return; }

        let max_eps = data.last().map(|(e, _)| *e).unwrap_or(1.0);
        let max_betti = data.iter().flat_map(|(_, bs)| bs.iter().copied()).fold(1, usize::max) as f64;

        let colors = ["#ff6b6b", "#6bcb77", "#4d96ff"];
        for (dim, color) in colors.iter().enumerate() {
            ctx.set_stroke_style(&JsValue::from_str(color));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            for (i, (eps, bettis)) in data.iter().enumerate() {
                let x = pad + (eps / max_eps) * plot_w;
                let val = bettis.get(dim).copied().unwrap_or(0) as f64;
                let y = pad + plot_h - (val / max_betti) * plot_h;
                if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
            }
            ctx.stroke();
        }

        // Legend
        ctx.set_font("11px monospace");
        for (i, (label, color)) in ["H₀", "H₁", "H₂"].iter().zip(colors.iter()).enumerate() {
            ctx.set_fill_style(&JsValue::from_str(color));
            ctx.fill_text(label, pad + 5.0, pad + 15.0 + i as f64 * 16.0);
        }
    });

    view! {
        <canvas _ref=canvas_ref width=width as u32 height=height as u32 class="betti-canvas"/>
    }
}
