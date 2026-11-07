mod components;

use components::win98::{ErrorMessage, Heading, Label, ResultDisplay, TextInput, Window};
use leptos::prelude::*;
use velocity_core::{check_half_days, SprintVelocities, VelocityError};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

const DEFAULT_VELOCITY: &str = "10";
const DEFAULT_ENGINEERS: &str = "10";
const DEFAULT_PRODUCTIVE_DAYS: &str = "10";
const DEFAULT_DAYS_OFF: &str = "25";

/// Checks the days off text is a whole or half day, or says what is wrong with it.
fn parse_days_off(text: &str) -> Result<f64, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("Days off is empty. Enter a number such as 1 or 1.5.".to_string());
    }
    let value = trimmed
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| "Days off must be a number such as 1 or 1.5.".to_string())?;
    check_half_days("days_off", value).map_err(|error| match error {
        VelocityError::NotHalfDay(_) => {
            "Days off must be a whole or half day, such as 1 or 1.5.".to_string()
        }
        _ => "Days off must be a number that is not negative.".to_string(),
    })?;
    Ok(value)
}

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
    let engineers = RwSignal::new(DEFAULT_ENGINEERS.to_string());
    let productive_days = RwSignal::new(DEFAULT_PRODUCTIVE_DAYS.to_string());
    let days_off = RwSignal::new(DEFAULT_DAYS_OFF.to_string());
    let days_off_error = Signal::derive(move || parse_days_off(&days_off.get()).err());
    let average =
        Signal::derive(move || average_text(&sprint1.get(), &sprint2.get(), &sprint3.get()));
    view! {
        <main class="win98-desktop">
            <Window id="velocity-window" title="Velocity Generator">
                <Heading>"Velocity Generator 🚀"</Heading>
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
                <div class="win98-field">
                    <Label for_id="engineers">"Number of Engineers 🤼"</Label>
                    <TextInput id="engineers" value=engineers />
                </div>
                <div class="win98-field">
                    <Label for_id="productive-days">"Number Of Productive Days 📆"</Label>
                    <TextInput id="productive-days" value=productive_days />
                </div>
                <div class="win98-field">
                    <Label for_id="days-off">"Number Of Days off 🏖"</Label>
                    <TextInput
                        id="days-off"
                        value=days_off
                        described_by="days-off-error"
                        invalid=Signal::derive(move || days_off_error.get().is_some())
                    />
                    {move || {
                        days_off_error
                            .get()
                            .map(|message| {
                                view! { <ErrorMessage id="days-off-error">{message}</ErrorMessage> }
                            })
                    }}
                </div>
            </Window>
        </main>
    }
}

#[cfg(test)]
mod tests {
    use super::{
        average_text, parse_days_off, DEFAULT_DAYS_OFF, DEFAULT_ENGINEERS, DEFAULT_PRODUCTIVE_DAYS,
    };

    #[test]
    fn defaults_days_off_to_25() {
        assert_eq!(DEFAULT_DAYS_OFF, "25");
        assert_eq!(parse_days_off(DEFAULT_DAYS_OFF), Ok(25.0));
    }

    #[test]
    fn accepts_whole_and_half_days() {
        for text in ["0", "1", "1.5", "2", " 2.5 "] {
            assert!(parse_days_off(text).is_ok(), "{text}");
        }
    }

    #[test]
    fn says_nan_and_infinity_are_not_numbers() {
        for text in ["NaN", "inf", "-inf", "infinity"] {
            assert_eq!(
                parse_days_off(text),
                Err("Days off must be a number such as 1 or 1.5.".to_string()),
                "{text}"
            );
        }
        assert_eq!(
            parse_days_off("-1"),
            Err("Days off must be a number that is not negative.".to_string())
        );
    }

    #[test]
    fn rejects_bad_days_off() {
        for text in ["", "abc", "-1", "1.3", "1.25", "NaN", "inf"] {
            assert!(parse_days_off(text).is_err(), "{text}");
        }
    }

    #[test]
    fn defaults_productive_days_to_ten() {
        assert_eq!(DEFAULT_PRODUCTIVE_DAYS, "10");
        assert!(DEFAULT_PRODUCTIVE_DAYS.parse::<f64>().is_ok());
    }

    #[test]
    fn defaults_engineers_to_ten() {
        assert_eq!(DEFAULT_ENGINEERS, "10");
    }

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
