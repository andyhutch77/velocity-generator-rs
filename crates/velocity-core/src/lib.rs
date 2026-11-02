//! Velocity calculation logic.
//!
//! This crate has no UI or browser dependencies, so it can be tested
//! natively with `cargo test` and reused by any front end.

/// Average story points completed per sprint.
///
/// Returns `None` when there are no sprints to average.
#[must_use]
pub fn average_velocity(completed_points: &[u32]) -> Option<f64> {
    if completed_points.is_empty() {
        return None;
    }
    let total: u64 = completed_points.iter().map(|&p| u64::from(p)).sum();
    #[allow(clippy::cast_precision_loss)]
    Some(total as f64 / completed_points.len() as f64)
}

/// Rounds to two decimals, with halves going towards positive infinity (like JS `Math.round`).
fn round_two_decimals(x: f64) -> f64 {
    (x * 100.0 + 0.5).floor() / 100.0
}

/// Story points completed in the three most recent sprints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SprintVelocities {
    /// Points completed in the first sprint.
    pub sprint1: f64,
    /// Points completed in the second sprint.
    pub sprint2: f64,
    /// Points completed in the third sprint.
    pub sprint3: f64,
}

impl SprintVelocities {
    /// Average of the three sprints, rounded to two decimals.
    #[must_use]
    pub fn average(&self) -> f64 {
        round_two_decimals((self.sprint1 + self.sprint2 + self.sprint3) / 3.0)
    }
}

impl Default for SprintVelocities {
    fn default() -> Self {
        Self {
            sprint1: 10.0,
            sprint2: 10.0,
            sprint3: 10.0,
        }
    }
}

/// Everything needed to suggest story points for the next sprint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VelocityInputs {
    /// Velocities of the previous three sprints.
    pub previous: SprintVelocities,
    /// Number of developers.
    pub developers: u32,
    /// Productive days per developer.
    pub productive_days: u32,
    /// Total days off across the team.
    pub days_off: f64,
    /// Points carried across from the previous sprint.
    pub points_carried_across: f64,
    /// Team capacity as a percentage, from 0 to 100.
    pub capacity_percent: f64,
}

impl Default for VelocityInputs {
    fn default() -> Self {
        Self {
            previous: SprintVelocities::default(),
            developers: 10,
            productive_days: 10,
            days_off: 25.0,
            points_carried_across: 0.0,
            capacity_percent: 100.0,
        }
    }
}

/// The figures shown for a velocity calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VelocityResult {
    /// Average of the previous three sprints.
    pub previous_average: f64,
    /// Suggested total story points, before carried points are subtracted.
    pub total_story_points: f64,
    /// Suggested new story points, after carried points are subtracted.
    pub new_story_points: f64,
}

/// Why a velocity calculation was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VelocityError {
    /// Developers times productive days is zero.
    NoCapacity,
    /// A value is NaN, infinite or negative; names the field.
    InvalidValue(&'static str),
    /// Days off are more than developers times productive days.
    DaysOffExceedCapacity,
    /// Capacity percent is above 100.
    CapacityOutOfRange,
}

impl std::fmt::Display for VelocityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCapacity => {
                write!(f, "developers and productive days must both be above zero")
            }
            Self::InvalidValue(field) => {
                write!(f, "{field} must be a number that is not negative")
            }
            Self::DaysOffExceedCapacity => {
                write!(
                    f,
                    "days off cannot be more than developers times productive days"
                )
            }
            Self::CapacityOutOfRange => write!(f, "capacity cannot be more than 100%"),
        }
    }
}

impl std::error::Error for VelocityError {}

fn check_value(field: &'static str, value: f64) -> Result<(), VelocityError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(VelocityError::InvalidValue(field))
    }
}

impl VelocityInputs {
    /// Calculates the suggested story points for the next sprint.
    ///
    /// # Errors
    ///
    /// Returns a [`VelocityError`] when a value is invalid, there is no
    /// capacity, days off exceed capacity, or capacity is above 100%.
    pub fn calculate(&self) -> Result<VelocityResult, VelocityError> {
        check_value("sprint1", self.previous.sprint1)?;
        check_value("sprint2", self.previous.sprint2)?;
        check_value("sprint3", self.previous.sprint3)?;
        check_value("days_off", self.days_off)?;
        check_value("points_carried_across", self.points_carried_across)?;
        check_value("capacity_percent", self.capacity_percent)?;

        let max = u64::from(self.developers) * u64::from(self.productive_days);
        if max == 0 {
            return Err(VelocityError::NoCapacity);
        }
        #[allow(clippy::cast_precision_loss)] // below 2^64, and exact enough for a day count
        let max = max as f64;
        if self.days_off > max {
            return Err(VelocityError::DaysOffExceedCapacity);
        }
        if self.capacity_percent > 100.0 {
            return Err(VelocityError::CapacityOutOfRange);
        }

        let previous_average = self.previous.average();
        let reduction = previous_average * (self.days_off / max);
        let total =
            round_two_decimals((previous_average - reduction) * self.capacity_percent / 100.0);
        let new_points = round_two_decimals(total - self.points_carried_across);
        Ok(VelocityResult {
            previous_average,
            total_story_points: total,
            new_story_points: new_points,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(f: impl FnOnce(&mut VelocityInputs)) -> VelocityInputs {
        let mut i = VelocityInputs::default();
        f(&mut i);
        i
    }

    fn sprints(a: f64, b: f64, c: f64) -> SprintVelocities {
        SprintVelocities {
            sprint1: a,
            sprint2: b,
            sprint3: c,
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 0.005, "{actual} != {expected}");
    }

    fn check(i: &VelocityInputs, total: f64, new: f64) {
        let r = i.calculate().expect("valid inputs");
        assert_close(r.total_story_points, total);
        assert_close(r.new_story_points, new);
    }

    #[test]
    fn default_inputs_match_original() {
        let i = VelocityInputs::default();
        assert_eq!(i.previous, sprints(10.0, 10.0, 10.0));
        assert_eq!(i.developers, 10);
        assert_eq!(i.productive_days, 10);
        assert_close(i.days_off, 25.0);
        assert_close(i.points_carried_across, 0.0);
        assert_close(i.capacity_percent, 100.0);
    }

    #[test]
    fn golden_example_defaults() {
        let r = VelocityInputs::default().calculate().expect("valid");
        assert_close(r.previous_average, 10.0);
        assert_close(r.total_story_points, 7.5);
        assert_close(r.new_story_points, 7.5);
    }

    #[test]
    fn average_of_three_sprints() {
        assert_close(sprints(10.0, 20.0, 30.0).average(), 20.0);
    }

    #[test]
    fn average_rounds_down_to_two_decimals() {
        assert_close(sprints(10.0, 10.0, 11.0).average(), 10.33);
    }

    #[test]
    fn average_rounds_up_to_two_decimals() {
        assert_close(sprints(10.0, 11.0, 11.0).average(), 10.67);
    }

    #[test]
    fn no_days_off_means_no_reduction() {
        check(&inputs(|i| i.days_off = 0.0), 10.0, 10.0);
    }

    #[test]
    fn reduction_uses_rounded_average() {
        let i = inputs(|i| {
            i.previous = sprints(10.0, 10.0, 11.0);
            i.days_off = 10.0;
        });
        check(&i, 9.3, 9.3);
    }

    #[test]
    fn half_capacity_halves_total() {
        check(&inputs(|i| i.capacity_percent = 50.0), 3.75, 3.75);
    }

    #[test]
    fn zero_capacity_gives_zero_total() {
        check(&inputs(|i| i.capacity_percent = 0.0), 0.0, 0.0);
    }

    #[test]
    fn carried_points_reduce_new_points() {
        check(&inputs(|i| i.points_carried_across = 3.0), 7.5, 4.5);
    }

    #[test]
    fn new_points_can_be_negative() {
        check(&inputs(|i| i.points_carried_across = 10.0), 7.5, -2.5);
    }

    #[test]
    fn new_points_round_after_subtracting() {
        check(&inputs(|i| i.points_carried_across = 0.125), 7.5, 7.38);
    }

    #[test]
    fn round_half_up() {
        assert_close(round_two_decimals(0.125), 0.13);
    }

    #[test]
    fn round_negative_half_towards_positive() {
        assert_close(round_two_decimals(-0.125), -0.12);
    }

    #[test]
    fn zero_developers_is_error() {
        let i = inputs(|i| i.developers = 0);
        assert_eq!(i.calculate(), Err(VelocityError::NoCapacity));
    }

    #[test]
    fn zero_productive_days_is_error() {
        let i = inputs(|i| i.productive_days = 0);
        assert_eq!(i.calculate(), Err(VelocityError::NoCapacity));
    }

    #[test]
    fn nan_is_invalid() {
        let i = inputs(|i| i.capacity_percent = f64::NAN);
        assert_eq!(
            i.calculate(),
            Err(VelocityError::InvalidValue("capacity_percent"))
        );
    }

    #[test]
    fn infinite_is_invalid() {
        let i = inputs(|i| i.previous.sprint1 = f64::INFINITY);
        assert_eq!(i.calculate(), Err(VelocityError::InvalidValue("sprint1")));
    }

    #[test]
    fn negative_is_invalid() {
        let i = inputs(|i| i.days_off = -1.0);
        assert_eq!(i.calculate(), Err(VelocityError::InvalidValue("days_off")));
    }

    #[test]
    fn days_off_above_capacity_is_error() {
        let i = inputs(|i| {
            i.developers = 1;
            i.productive_days = 1;
            i.days_off = 2.0;
        });
        assert_eq!(i.calculate(), Err(VelocityError::DaysOffExceedCapacity));
    }

    #[test]
    fn days_off_equal_to_capacity_gives_zero() {
        check(&inputs(|i| i.days_off = 100.0), 0.0, 0.0);
    }

    #[test]
    fn capacity_above_100_is_error() {
        let i = inputs(|i| i.capacity_percent = 101.0);
        assert_eq!(i.calculate(), Err(VelocityError::CapacityOutOfRange));
    }

    #[test]
    fn error_messages_are_readable() {
        let errors = [
            VelocityError::NoCapacity,
            VelocityError::InvalidValue("days_off"),
            VelocityError::DaysOffExceedCapacity,
            VelocityError::CapacityOutOfRange,
        ];
        for e in errors {
            assert!(!e.to_string().is_empty());
            let _: &dyn std::error::Error = &e;
        }
    }

    #[test]
    fn no_sprints_gives_none() {
        assert_eq!(average_velocity(&[]), None);
    }

    #[test]
    fn averages_completed_points() {
        assert_eq!(average_velocity(&[20, 25, 30]), Some(25.0));
    }
}
