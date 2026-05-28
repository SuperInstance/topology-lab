use leptos::*;

/// I Ching hexagram with sheaf cohomology data
#[component]
pub fn HexagramView(
    lines: Vec<bool>,
    number: u8,
    name: String,
    h0: f64,
    h1: f64,
) -> impl IntoView {
    view! {
        <div class="hexagram-view">
            <div class="hexagram-number">{format!("#{}", number)}</div>
            <div class="hexagram-name">{name}</div>
            <div class="hexagram-lines">
                {lines.iter().rev().enumerate().map(|(i, &yang)| {
                    if yang {
                        view! {
                            <div class="hex-line yang" key=i>
                                <div class="yang-bar"></div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="hex-line yin" key=i>
                                <div class="yin-bar"></div>
                                <div class="yin-gap"></div>
                                <div class="yin-bar"></div>
                            </div>
                        }.into_view()
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="sheaf-data">
                <div class="sheaf-row">
                    <span class="sheaf-label">"H0 (agreement)"</span>
                    <span class="sheaf-value" style=format!("color: {}", if h0 > 0.5 { "#6bcb77" } else { "#ffd93d" })>
                        {format!("{:.3}", h0)}
                    </span>
                </div>
                <div class="sheaf-row">
                    <span class="sheaf-label">"H1 (wisdom)"</span>
                    <span class="sheaf-value" style=format!("color: {}", if h1 > 0.3 { "#ff6b6b" } else { "#4d96ff" })>
                        {format!("{:.3}", h1)}
                    </span>
                </div>
            </div>
        </div>
    }
}
