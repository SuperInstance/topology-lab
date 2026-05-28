use leptos::*;
use leptos_router::A;

#[component]
pub fn Nav() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <nav class="sidebar" class:open=move || open.get()>
            <div class="sidebar-header">
                <A href="/" class="logo" on:click=move |_| set_open.set(false)>
                    <span class="logo-icon">{"\u{222c}"}</span>
                    <span class="logo-text">"Topology Lab"</span>
                </A>
            </div>
            <button class="menu-toggle" on:click=move |_| set_open.update(|v| *v = !*v)>
                {move || if open.get() { "\u{2715}" } else { "\u{2630}" }}
            </button>
            <ul class="nav-links">
                <li><A href="/" on:click=move |_| set_open.set(false)>"\u{1f3e0} Home"</A></li>
                <li><A href="/persistence" on:click=move |_| set_open.set(false)>"\u{1f535} Persistence Lab"</A></li>
                <li><A href="/symplectic" on:click=move |_| set_open.set(false)>"\u{1f300} Symplectic"</A></li>
                <li><A href="/iching" on:click=move |_| set_open.set(false)>"\u{262f} I Ching Oracle"</A></li>
                <li><A href="/music" on:click=move |_| set_open.set(false)>"\u{266b} Music"</A></li>
                <li><A href="/social" on:click=move |_| set_open.set(false)>"\u{1f578} Social"</A></li>
                <li><A href="/conjectures" on:click=move |_| set_open.set(false)>"\u{1f4cb} Conjectures"</A></li>
            </ul>
            <div class="sidebar-footer">
                <a href="https://github.com/SuperInstance" target="_blank">"SuperInstance"</a>
            </div>
        </nav>
    }
}
