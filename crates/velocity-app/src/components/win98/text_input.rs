use leptos::prelude::*;

/// A single-line text input with a sunken bevel. Pair it with a `Label` using the same id.
#[component]
pub fn TextInput(
    #[prop(into)] id: String,
    value: RwSignal<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <input
            id=id
            class="win98-text-input"
            type="text"
            disabled=disabled
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
