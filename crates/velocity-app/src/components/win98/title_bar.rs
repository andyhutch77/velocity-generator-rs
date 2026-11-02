use leptos::prelude::*;

/// Navy title bar. Renders the page heading, so use it once per page.
#[component]
pub fn TitleBar(
    /// Id of the heading, referenced by the window's `aria-labelledby`.
    #[prop(into)]
    id: String,
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <header class="win98-title-bar">
            <h1 id=id class="win98-title-bar__text">{title}</h1>
        </header>
    }
}
