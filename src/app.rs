use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::nav::Nav;
use crate::pages::home::HomePage;
use crate::pages::persistence::PersistencePage;
use crate::pages::symplectic::SymplecticPage;
use crate::pages::iching::IChingPage;
use crate::pages::music::MusicPage;
use crate::pages::social::SocialPage;
use crate::pages::conjectures::ConjecturesPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/topology-lab.css"/>
        <Title text="Topology Lab"/>
        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <div class="app-layout">
            <Nav/>
            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/persistence" view=PersistencePage/>
                    <Route path="/symplectic" view=SymplecticPage/>
                    <Route path="/iching" view=IChingPage/>
                    <Route path="/music" view=MusicPage/>
                    <Route path="/social" view=SocialPage/>
                    <Route path="/conjectures" view=ConjecturesPage/>
                </Routes>
            </main>
        </div>
    }
}
