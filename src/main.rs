use leptos::*;
use leptos_router::*;
use topology_lab::App;

fn main() {
    console_log::init_with_level(log::Level::Debug).ok();
    mount_to_body(|| {
        view! {
            <Router>
                <App/>
            </Router>
        }
    });
}
