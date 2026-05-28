use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use leptos::*;

/// Interactive persistence diagram (birth, death) scatter plot
#[component]
pub fn PersistenceDiagram(
    points: Vec<(f64, f64)>,
    max_val: f64,
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

        // Background
        ctx.set_fill_style(&JsValue::from_str("#1a1a2e"));
        ctx.fill_rect(0.0, 0.0, w, h);

        // Grid
        ctx.set_stroke_style(&JsValue::from_str("#2a2a4e"));
        ctx.set_line_width(0.5);
        for i in 0..=10 {
            let x = pad + (plot_w * i as f64 / 10.0);
            let y = pad + (plot_h * i as f64 / 10.0);
            ctx.begin_path();
            ctx.move_to(x, pad);
            ctx.line_to(x, pad + plot_h);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(pad, y);
            ctx.line_to(pad + plot_w, y);
            ctx.stroke();
        }

        // Diagonal
        ctx.set_stroke_style(&JsValue::from_str("#4a4a7e"));
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(pad, pad + plot_h);
        ctx.line_to(pad + plot_w, pad);
        ctx.stroke();

        // Points
        for (b, d) in &points {
            let x = pad + (b / max_val) * plot_w;
            let y = pad + plot_h - (d / max_val) * plot_h;
            let persistence = d - b;

            let color = if persistence > max_val * 0.3 {
                "#ff6b6b"
            } else if persistence > max_val * 0.1 {
                "#ffd93d"
            } else {
                "#6bcb77"
            };

            ctx.set_fill_style(&JsValue::from_str(color));
            ctx.begin_path();
            ctx.arc(x, y, 4.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.fill();
        }

        // Axes labels
        ctx.set_fill_style(&JsValue::from_str("#aaaacc"));
        ctx.set_font("12px monospace");
        ctx.fill_text("Birth", pad + plot_w / 2.0 - 15.0, h - 5.0);
        ctx.save();
        ctx.translate(12.0, pad + plot_h / 2.0).unwrap();
        ctx.rotate(-std::f64::consts::FRAC_PI_2).unwrap();
        ctx.fill_text("Death", -15.0, 0.0);
        ctx.restore();
    });

    view! {
        <canvas
            _ref=canvas_ref
            width=width as u32
            height=height as u32
            class="persistence-canvas"
        />
    }
}
