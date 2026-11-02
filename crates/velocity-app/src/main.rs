mod components;

use components::win98::{Button, Label, TextInput, Window};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

/// Placeholder page built from the Windows 98 component library.
/// The real calculator UI is separate work.
#[component]
fn App() -> impl IntoView {
    let name = RwSignal::new(String::new());
    view! {
        <main class="win98-desktop">
            <Window id="velocity-window" title="Velocity Calculator">
                <div class="win98-field">
                    <Label for_id="demo-input">"Sprint name"</Label>
                    <TextInput id="demo-input" value=name />
                </div>
                <Button>"OK"</Button>
            </Window>
        </main>
    }
}
