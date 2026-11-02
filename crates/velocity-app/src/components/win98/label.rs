use leptos::prelude::*;

/// A text label tied to the control with the given id.
#[component]
pub fn Label(#[prop(into)] for_id: String, children: Children) -> impl IntoView {
    view! {
        <label class="win98-label" for=for_id>
            {children()}
        </label>
    }
}
