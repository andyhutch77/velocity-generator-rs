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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_sprints_gives_none() {
        assert_eq!(average_velocity(&[]), None);
    }

    #[test]
    fn averages_completed_points() {
        assert_eq!(average_velocity(&[20, 25, 30]), Some(25.0));
    }
}
