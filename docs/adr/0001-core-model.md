# 0001: Core model for the velocity calculation

Status: proposed. Analysis only; no code changes. Source analysed: `src/VelocityGenerator.js` in the original React `velocity-calculator`.

## Context

The original app takes sprint history, team size and time off, and shows a
suggested story point total for the next sprint. This note records the model,
formulas, edge-case decisions and tests needed to rebuild it in `velocity-core`.
Follow-up issues implement it.

## Original behaviour

Inputs and defaults: three previous sprint velocities (10, 10, 10), developers
(10), productive days per developer (10), days off (25), points carried across
(0), capacity percent (100).

```text
previous_average = round2((s1 + s2 + s3) / 3)
max              = developers * productive_days
reduction        = previous_average * (days_off / max)      // not rounded
total            = round2((previous_average - reduction) * capacity_percent / 100)
new_points       = round2(total - points_carried_across)    // uses rounded total
round2(x)        = Math.round(x * 100) / 100                // halves round towards +infinity
```

The UI shows only `previous_average`, `total` and `new_points`.

Accidental behaviour in the original: `max = 0` gives Infinity/NaN; unparsable
input gives NaN (`parseInt`); days off above `max` or capacity above 100 give
negative or inflated results.

## Proposed model (all in `velocity-core`, no UI or browser code)

Types take typed numbers. Parsing text into numbers stays in `velocity-app`.

```rust
/// Story points completed in the three most recent sprints.
pub struct SprintVelocities { pub sprint1: f64, pub sprint2: f64, pub sprint3: f64 }
impl SprintVelocities {
    pub fn average(&self) -> f64;            // round2((s1 + s2 + s3) / 3)
}
impl Default for SprintVelocities { /* 10, 10, 10 */ }

pub struct VelocityInputs {
    pub previous: SprintVelocities,          // default 10, 10, 10
    pub developers: u32,                     // default 10
    pub productive_days: u32,                // default 10, per developer
    pub days_off: f64,                       // default 25
    pub points_carried_across: f64,          // default 0
    pub capacity_percent: f64,               // default 100
}
impl Default for VelocityInputs { /* values above */ }
impl VelocityInputs {
    pub fn calculate(&self) -> Result<VelocityResult, VelocityError>;
}

pub struct VelocityResult {
    pub previous_average: f64,
    pub total_story_points: f64,
    pub new_story_points: f64,
}

#[non_exhaustive]
pub enum VelocityError {
    NoCapacity,                 // developers * productive_days == 0
    InvalidValue(&'static str), // NaN, infinite or negative input; names the field
    DaysOffExceedCapacity,      // days_off > developers * productive_days
    CapacityOutOfRange,         // capacity_percent > 100
}
// Display + std::error::Error implemented; no panics, no unwrap/expect.

// private; (x * 100.0 + 0.5).floor() / 100.0, matching JS Math.round
fn round_two_decimals(x: f64) -> f64;
```

The existing `average_velocity(&[u32]) -> Option<f64>` stays as it is; the new
model is added beside it, not instead of it.

## Decisions on edge cases

| Case | Decision |
| --- | --- |
| Zero developers or zero productive days | `Err(NoCapacity)`. The original divides by zero. |
| NaN, infinite or negative value in any field | `Err(InvalidValue(field))`. The app turns it into a message. |
| Developer and day counts | `u32`, so negatives cannot occur. Parsing lives in the app. |
| Days off above `developers * productive_days` | `Err(DaysOffExceedCapacity)`. The original gives a negative total. |
| Capacity above 100% | `Err(CapacityOutOfRange)`. The original inflates the total. Capacity of 0 is allowed and gives total 0. |
| Points carried across above total | Allowed. `new_points` can be negative, as in the original; the UI decides how to show it. |
| Rounding | Round half towards +infinity at two decimals, as `Math.round`. `reduction` is not rounded. `total` is rounded before subtracting carried points. |
| Decimals | Sprint velocities, days off, carried points and capacity accept decimals. Counts are whole numbers. |

These rejections are a deliberate change from the original's quirks; the
maintainer should confirm them.

## Test list (unit tests in `crates/velocity-core`, expected values worked by hand)

| Test name | Input | Expected |
| --- | --- | --- |
| `default_inputs_match_original` | `VelocityInputs::default()` | sprints 10/10/10, 10 devs, 10 days, 25 days off, 0 carried, 100% |
| `golden_example_defaults` | defaults | average 10, total 7.5, new 7.5 (reduction 10 * 25/100 = 2.5) |
| `average_of_three_sprints` | 10, 20, 30 | 20 |
| `average_rounds_down_to_two_decimals` | 10, 10, 11 | 10.33 |
| `average_rounds_up_to_two_decimals` | 10, 11, 11 | 10.67 |
| `no_days_off_means_no_reduction` | defaults, days_off 0 | total 10, new 10 |
| `reduction_uses_rounded_average` | 10, 10, 11; days_off 10; 10 devs * 10 days | reduction 10.33 * 0.1 = 1.033; total round2(9.297) = 9.3 |
| `half_capacity_halves_total` | defaults, capacity 50 | total 3.75, new 3.75 |
| `zero_capacity_gives_zero_total` | capacity 0 | total 0, new 0 |
| `carried_points_reduce_new_points` | defaults, carried 3 | total 7.5, new 4.5 |
| `new_points_can_be_negative` | defaults, carried 10 | total 7.5, new -2.5 |
| `new_points_round_after_subtracting` | defaults, carried 0.125 | total 7.5, new round2(7.375) = 7.38 |
| `round_half_up` | round2(0.125) | 0.13 |
| `round_negative_half_towards_positive` | round2(-0.125) | -0.12 |
| `zero_developers_is_error` | developers 0 | `Err(NoCapacity)` |
| `zero_productive_days_is_error` | productive_days 0 | `Err(NoCapacity)` |
| `nan_is_invalid` | capacity NaN | `Err(InvalidValue("capacity_percent"))` |
| `infinite_is_invalid` | sprint1 infinity | `Err(InvalidValue("sprint1"))` |
| `negative_is_invalid` | days_off -1 | `Err(InvalidValue("days_off"))` |
| `days_off_above_capacity_is_error` | 1 dev, 1 day, days_off 2 | `Err(DaysOffExceedCapacity)` |
| `days_off_equal_to_capacity_gives_zero` | 10 devs, 10 days, days_off 100 | total 0 |
| `capacity_above_100_is_error` | capacity 101 | `Err(CapacityOutOfRange)` |
| `error_messages_are_readable` | each error | non-empty `Display` text |

Floating-point note: values that sit exactly on a half (such as 1.005) may round
differently from JavaScript because of binary representation. Tests use values
that are exact in binary (such as 0.125) and compare results to two decimals.

End-to-end tests (Playwright, in `e2e/`, for the app issue): the default state
shows 10 / 7.5 / 7.5, and editing an input updates the results.

## Consequences

Follow-up issues can implement the model test-first from the list above. The
decisions marked for confirmation may change before then.
