use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

/// Placeholder shell: one empty Windows 98-style window.
/// The real UI and component library are built by the agents.
#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <section class="window" aria-labelledby="window-title">
                <header class="title-bar">
                    <h1 id="window-title">"Velocity Calculator"</h1>
                </header>
                <div class="window-body">
                    <p>"Coming soon."</p>
                </div>
            </section>
        </main>
    }
}
