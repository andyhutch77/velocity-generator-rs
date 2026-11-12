mod components;

use components::win98::{ErrorMessage, Heading, Label, ResultDisplay, TextInput, Window};
use leptos::prelude::*;
use velocity_core::{
    check_half_days, check_half_points, SprintVelocities, VelocityError, VelocityInputs,
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

const DEFAULT_VELOCITY: &str = "10";
const DEFAULT_ENGINEERS: &str = "10";
const DEFAULT_PRODUCTIVE_DAYS: &str = "10";
const DEFAULT_DAYS_OFF: &str = "25";
const DEFAULT_POINTS_CARRIED: &str = "0";
const NO_NEW_POINTS: &str = "-";

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

/// Checks the points carried across text is a whole or half number, or says what is wrong with it.
fn parse_points_carried(text: &str) -> Result<f64, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("Points carried across is empty. Enter a number such as 1 or 1.5.".to_string());
    }
    let value = trimmed
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| "Points carried across must be a number such as 1 or 1.5.".to_string())?;
    check_half_points("points_carried_across", value).map_err(|error| match error {
        VelocityError::NotHalfPoints(_) => {
            "Points carried across must be a whole or half number, such as 1 or 1.5.".to_string()
        }
        _ => "Points carried across must be a number that is not negative.".to_string(),
    })?;
    Ok(value)
}

/// New story points for the sprint, or a dash while any input is not yet valid
/// (each input explains its own problem).
fn new_story_points_text(
    sprints: [&str; 3],
    engineers: &str,
    productive_days: &str,
    days_off: &str,
    points_carried: &str,
) -> String {
    let calculate = || -> Option<f64> {
        let [s1, s2, s3] = sprints;
        let inputs = VelocityInputs {
            previous: SprintVelocities {
                sprint1: parse_sprint("Sprint 1", s1).ok()?,
                sprint2: parse_sprint("Sprint 2", s2).ok()?,
                sprint3: parse_sprint("Sprint 3", s3).ok()?,
            },
            developers: engineers.trim().parse().ok()?,
            productive_days: productive_days.trim().parse().ok()?,
            days_off: parse_days_off(days_off).ok()?,
            points_carried_across: parse_points_carried(points_carried).ok()?,
            ..VelocityInputs::default()
        };
        inputs
            .calculate()
            .ok()
            .map(|result| result.new_story_points)
    };
    calculate().map_or_else(|| NO_NEW_POINTS.to_string(), |points| points.to_string())
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
    let points_carried = RwSignal::new(DEFAULT_POINTS_CARRIED.to_string());
    let points_carried_error =
        Signal::derive(move || parse_points_carried(&points_carried.get()).err());
    let new_points = Signal::derive(move || {
        Ok(new_story_points_text(
            [&sprint1.get(), &sprint2.get(), &sprint3.get()],
            &engineers.get(),
            &productive_days.get(),
            &days_off.get(),
            &points_carried.get(),
        ))
    });
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
                <div class="win98-field">
                    <Label for_id="points-carried">"Points Carried Across"</Label>
                    <TextInput
                        id="points-carried"
                        value=points_carried
                        described_by="points-carried-error"
                        invalid=Signal::derive(move || points_carried_error.get().is_some())
                    />
                    {move || {
                        points_carried_error
                            .get()
                            .map(|message| {
                                view! {
                                    <ErrorMessage id="points-carried-error">{message}</ErrorMessage>
                                }
                            })
                    }}
                </div>
                <ResultDisplay id="new-points" label="New story points" value=new_points />
            </Window>
        </main>
    }
}

#[cfg(test)]
mod tests {
    use super::{
        average_text, new_story_points_text, parse_days_off, parse_points_carried,
        DEFAULT_DAYS_OFF, DEFAULT_ENGINEERS, DEFAULT_POINTS_CARRIED, DEFAULT_PRODUCTIVE_DAYS,
    };

    #[test]
    fn defaults_points_carried_to_zero() {
        assert_eq!(DEFAULT_POINTS_CARRIED, "0");
        assert_eq!(parse_points_carried(DEFAULT_POINTS_CARRIED), Ok(0.0));
    }

    #[test]
    fn accepts_whole_and_half_points() {
        for (text, value) in [
            ("0", 0.0),
            ("0.5", 0.5),
            ("1", 1.0),
            (" 1.5 ", 1.5),
            ("2", 2.0),
        ] {
            assert_eq!(parse_points_carried(text), Ok(value), "{text}");
        }
    }

    #[test]
    fn rejects_bad_points_carried_with_messages() {
        for text in ["", "abc", "-1", "1.3", "0.25", "NaN", "inf", "-inf"] {
            assert!(parse_points_carried(text).is_err(), "{text}");
        }
        assert!(parse_points_carried("1.3")
            .unwrap_err()
            .contains("whole or half"));
        assert!(parse_points_carried("-1")
            .unwrap_err()
            .contains("not negative"));
        assert!(parse_points_carried("NaN")
            .unwrap_err()
            .contains("must be a number such as"));
        assert!(parse_points_carried("").unwrap_err().contains("empty"));
    }

    #[test]
    fn carried_points_reduce_new_story_points() {
        let text = |carried| new_story_points_text(["10", "10", "10"], "10", "10", "25", carried);
        assert_eq!(text("0"), "7.5");
        assert_eq!(text("2"), "5.5");
        assert_eq!(text("1.3"), "-");
    }

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
