mod components;

use components::win98::{Heading, Label, ResultDisplay, TextInput, Window};
use leptos::prelude::*;
use velocity_core::{SprintVelocities, VelocityError};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

const DEFAULT_VELOCITY: &str = "10";

/// Turns the text of one sprint box into a number, or says what is wrong with it.
fn parse_sprint(name: &str, text: &str) -> Result<f64, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(format!("{name} velocity is empty. Enter a number."));
    }
    trimmed
        .parse::<f64>()
        .map_err(|_| format!("{name} velocity must be a number."))
}

/// Average of the three sprint texts, formatted for display, or a plain-English error.
fn average_text(s1: &str, s2: &str, s3: &str) -> Result<String, String> {
    let sprints = SprintVelocities {
        sprint1: parse_sprint("Sprint 1", s1)?,
        sprint2: parse_sprint("Sprint 2", s2)?,
        sprint3: parse_sprint("Sprint 3", s3)?,
    };
    sprints
        .checked_average()
        .map(|average| average.to_string())
        .map_err(|error| match error {
            VelocityError::InvalidValue(field) => {
                let name = field.replace("sprint", "Sprint ");
                format!("{name} velocity must be a number that is not negative.")
            }
            _ => "The average could not be calculated.".to_string(),
        })
}

/// Shows the average velocity of the previous three sprints.
#[component]
fn App() -> impl IntoView {
    let sprint1 = RwSignal::new(DEFAULT_VELOCITY.to_string());
    let sprint2 = RwSignal::new(DEFAULT_VELOCITY.to_string());
    let sprint3 = RwSignal::new(DEFAULT_VELOCITY.to_string());
    let average =
        Signal::derive(move || average_text(&sprint1.get(), &sprint2.get(), &sprint3.get()));
    view! {
        <main class="win98-desktop">
            <Window id="velocity-window" title="Velocity Generator">
                <Heading>"Velocity Generator"</Heading>
                <div class="win98-field">
                    <Label for_id="sprint1">"Sprint 1 velocity"</Label>
                    <TextInput id="sprint1" value=sprint1 />
                </div>
                <div class="win98-field">
                    <Label for_id="sprint2">"Sprint 2 velocity"</Label>
                    <TextInput id="sprint2" value=sprint2 />
                </div>
                <div class="win98-field">
                    <Label for_id="sprint3">"Sprint 3 velocity"</Label>
                    <TextInput id="sprint3" value=sprint3 />
                </div>
                <ResultDisplay id="average" label="Average velocity" value=average />
            </Window>
        </main>
    }
}

#[cfg(test)]
mod tests {
    use super::average_text;

    #[test]
    fn defaults_average_to_ten() {
        assert_eq!(average_text("10", "10", "10"), Ok("10".to_string()));
    }

    #[test]
    fn rounds_to_two_decimals() {
        assert_eq!(average_text("10", "10", "11"), Ok("10.33".to_string()));
    }

    #[test]
    fn rejects_empty_text_and_words() {
        assert!(average_text("", "10", "10").is_err());
        assert!(average_text("10", "abc", "10").is_err());
    }

    #[test]
    fn rejects_negative_and_non_finite_values() {
        assert!(average_text("10", "10", "-1")
            .unwrap_err()
            .contains("Sprint 3"));
        assert!(average_text("NaN", "10", "10").is_err());
        assert!(average_text("inf", "10", "10").is_err());
    }
}
