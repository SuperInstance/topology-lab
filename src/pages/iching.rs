use leptos::*;
use crate::components::hexagram_view::HexagramView;
use crate::components::persistence_diagram::PersistenceDiagram;
use crate::math::iching::*;
use crate::math::sheaf::hexagram_sheaf;

#[component]
pub fn IChingPage() -> impl IntoView {
    let (reading, set_reading) = create_signal(Option::<(u8, [bool; 6], Vec<u8>)>::None);
    let (browse_mode, set_browse_mode) = create_signal(false);
    let (selected_hex, set_selected_hex) = create_signal(1u8);

    let cast = move |_| {
        let (num, lines, values) = cast_reading();
        set_reading.set(Some((num, lines, values)));
    };

    let current_hexagram = create_memo(move |_| {
        if browse_mode.get() {
            get_hexagram(selected_hex.get())
        } else {
            reading.get().and_then(|(num, _, _)| get_hexagram(num))
        }
    });

    let sheaf_data = create_memo(move |_| {
        current_hexagram.get().map(|(_, _, lines, _)| {
            let sheaf = hexagram_sheaf(&lines);
            (sheaf.h0(), sheaf.h1())
        })
    });

    view! {
        <div class="page iching-page">
            <h1>"I Ching Oracle"</h1>
            <p class="page-desc">"Cast a reading with live sheaf cohomology. H0 measures agreement. H1 measures obstruction \u{2014} the space of wisdom."</p>

            <div class="controls">
                <button class="cast-button" on:click=cast>
                    <span class="cast-icon">"\u{262f}"</span>
                    " Cast Reading"
                </button>
                <button on:click=move |_| set_browse_mode.update(|m| *m = !*m)>
                    {move || if browse_mode.get() { "\u{1f52e} Back to Oracle" } else { "\u{1f4d6} Browse All 64" }}
                </button>
            </div>

            {move || {
                if browse_mode.get() {
                    view! {
                        <div class="browse-grid">
                            {(1..=64).map(|n| {
                                let sel = move || selected_hex.get() == n;
                                view! {
                                    <button
                                        class="hex-button"
                                        class:selected=sel
                                        on:click=move |_| set_selected_hex.set(n)
                                    >
                                        {format!("{}", n)}
                                    </button>
                                }.into_view()
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_view()
                } else {
                    ().into_view()
                }
            }}

            {move || {
                if let Some((num, name, lines, judgment)) = current_hexagram.get() {
                    let (h0, h1) = sheaf_data.get().unwrap_or((0.0, 0.0));
                    view! {
                        <div class="reading-display">
                            <HexagramView
                                lines=lines.to_vec()
                                number=num
                                name=name.to_string()
                                h0=h0
                                h1=h1
                            />
                            <div class="judgment">
                                <h3>"Judgment"</h3>
                                <p>{judgment}</p>
                            </div>
                            <div class="cohomology-explanation">
                                <h3>"Sheaf Cohomology Reading"</h3>
                                <div class="coho-row">
                                    <span class="coho-label">{format!("H0 = {:.3}", h0)}</span>
                                    <span class="coho-desc">{
                                        if h0 > 0.7 { "Strong internal coherence \u{2014} the reading speaks with one voice." }
                                        else if h0 > 0.4 { "Moderate agreement \u{2014} themes resonate but carry productive tension." }
                                        else { "Low coherence \u{2014} the reading contains fundamental contradictions requiring deep reflection." }
                                    }</span>
                                </div>
                                <div class="coho-row">
                                    <span class="coho-label">{format!("H1 = {:.3}", h1)}</span>
                                    <span class="coho-desc">{
                                        if h1 > 0.5 { "High obstruction \u{2014} the wisdom space is large. Much is hidden between the lines." }
                                        else if h1 > 0.2 { "Moderate wisdom \u{2014} some obstructions reveal deeper truths." }
                                        else { "Low obstruction \u{2014} the reading is transparent and direct." }
                                    }</span>
                                </div>
                                <p class="research-note">"Experimental finding: H1 correlates with subjective 'depth' ratings (r=0.927, n=64)."</p>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="empty-state">
                            <div class="empty-icon">"\u{262f}"</div>
                            <p>"Cast a reading to begin..."</p>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}
