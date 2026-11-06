use leptos::prelude::*;

/// Large heading for the body of a window. Renders an `h2`; the title bar owns the `h1`.
#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! { <h2 class="win98-heading">{children()}</h2> }
}
