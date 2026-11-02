use super::TitleBar;
use leptos::prelude::*;

/// Id for a window's heading, derived from the window's id.
fn heading_id(id: &str) -> String {
    format!("{id}-title")
}

/// A raised, bevelled window with a title bar and a body.
#[component]
pub fn Window(
    #[prop(into)] id: String,
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    let title_id = heading_id(&id);
    let labelled_by = title_id.clone();
    view! {
        <section id=id class="win98-window" aria-labelledby=labelled_by>
            <TitleBar id=title_id title=title />
            <div class="win98-window__body">{children()}</div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::heading_id;

    #[test]
    fn heading_id_is_derived_from_window_id() {
        assert_eq!(heading_id("main"), "main-title");
    }
}
