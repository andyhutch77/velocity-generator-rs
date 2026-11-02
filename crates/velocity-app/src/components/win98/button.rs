use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// A bevelled push button that sinks when pressed.
#[component]
pub fn Button(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="win98-button"
            disabled=disabled
            on:click=move |ev| {
                if let Some(callback) = on_click {
                    callback.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
