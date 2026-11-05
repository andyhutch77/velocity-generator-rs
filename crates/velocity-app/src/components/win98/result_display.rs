use leptos::prelude::*;

/// A sunken read-only readout with a label. An `Err` shows its message in text,
/// announced to assistive technology, instead of a value.
#[component]
pub fn ResultDisplay(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<Result<String, String>>,
) -> impl IntoView {
    let label_id = format!("{id}-label");
    let labelled_by = label_id.clone();
    view! {
        <div id=id class="win98-result">
            <span id=label_id class="win98-label">
                {label}
            </span>
            {move || match value.get() {
                Ok(text) => {
                    view! {
                        <output class="win98-result__value" aria-labelledby=labelled_by.clone()>
                            {text}
                        </output>
                    }
                        .into_any()
                }
                Err(message) => {
                    view! {
                        <p class="win98-result__error" role="alert">
                            <span aria-hidden="true">"! "</span>
                            {message}
                        </p>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
