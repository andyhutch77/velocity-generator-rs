use leptos::prelude::*;

/// A plain-text error for one field, announced to assistive technology. Link it to
/// its input with the same id through `TextInput`'s `described_by`.
#[component]
pub fn ErrorMessage(#[prop(into)] id: String, children: Children) -> impl IntoView {
    view! {
        <p id=id class="win98-error-message" role="alert">
            <span aria-hidden="true">"! "</span>
            {children()}
        </p>
    }
}
