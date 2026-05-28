use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use leptos::*;

/// Music tension curve plot
#[component]
pub fn TensionCurve(
    tension_data: Vec<(f64, f64)>, // (time, tension)
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

        if tension_data.len() < 2 { return; }

        let t_max = tension_data.last().map(|(t, _)| *t).unwrap_or(1.0);
        let v_max = tension_data.iter().map(|(_, v)| *v).fold(1.0, f64::max) * 1.1;

        // Tension curve
        ctx.set_stroke_style(&JsValue::from_str("#ff6b6b"));
        ctx.set_line_width(2.5);
        ctx.begin_path();
        for (i, (t, v)) in tension_data.iter().enumerate() {
            let x = pad + (t / t_max) * plot_w;
            let y = pad + plot_h - (v / v_max) * plot_h;
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();

        // Gradient (dT/dt)
        if tension_data.len() > 2 {
            ctx.set_stroke_style(&JsValue::from_str("#ffd93d"));
            ctx.set_line_width(1.5);
            ctx.begin_path();
            for (i, w) in tension_data.windows(2).enumerate() {
                let dt = w[1].0 - w[0].0;
                let dv = w[1].1 - w[0].1;
                let grad = if dt.abs() > 1e-10 { dv / dt } else { 0.0 };
                let x = pad + (w[0].0 / t_max) * plot_w;
                let y = pad + plot_h / 2.0 - (grad / v_max) * plot_h * 0.5;
                if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
            }
            ctx.stroke();
        }

        // Labels
        ctx.set_fill_style(&JsValue::from_str("#aaaacc"));
        ctx.set_font("12px monospace");
        ctx.fill_text("Time", pad + plot_w / 2.0 - 15.0, height - 8.0);
        ctx.save();
        ctx.translate(12.0, pad + plot_h / 2.0).unwrap();
        ctx.rotate(-std::f64::consts::FRAC_PI_2).unwrap();
        ctx.fill_text("Tension", -20.0, 0.0);
        ctx.restore();

        // Legend
        ctx.set_font("11px monospace");
        ctx.set_fill_style(&JsValue::from_str("#ff6b6b"));
        ctx.fill_text("T(t)", pad + plot_w - 60.0, pad + 15.0);
        ctx.set_fill_style(&JsValue::from_str("#ffd93d"));
        ctx.fill_text("dT/dt", pad + plot_w - 60.0, pad + 30.0);
    });

    view! {
        <canvas _ref=canvas_ref width=width as u32 height=height as u32 class="tension-canvas"/>
    }
}
