use spacetimedb::Timestamp;
use std::time::Duration;

const SECS_PER_MINUTE: u64 = 60;
const MINS_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const DAYS_PER_WEEK: u64 = 7;

pub trait DurationExt {
    fn from_weeks_ext(weeks: u64) -> Self;
    fn from_days_ext(days: u64) -> Self;
    fn from_hours_ext(hours: u64) -> Self;
    fn from_mins_ext(minutes: u64) -> Self;
}

impl DurationExt for Duration {
    #[inline]
    fn from_weeks_ext(weeks: u64) -> Duration {
        if weeks > u64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR * HOURS_PER_DAY * DAYS_PER_WEEK) {
            panic!("overflow in Duration::from_weeks");
        }

        Duration::from_secs(weeks * MINS_PER_HOUR * SECS_PER_MINUTE * HOURS_PER_DAY * DAYS_PER_WEEK)
    }

    #[inline]
    fn from_days_ext(days: u64) -> Duration {
        if days > u64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR * HOURS_PER_DAY) {
            panic!("overflow in Duration::from_days");
        }

        Duration::from_secs(days * MINS_PER_HOUR * SECS_PER_MINUTE * HOURS_PER_DAY)
    }

    #[inline]
    fn from_hours_ext(hours: u64) -> Duration {
        if hours > u64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR) {
            panic!("overflow in Duration::from_hours");
        }

        Duration::from_secs(hours * MINS_PER_HOUR * SECS_PER_MINUTE)
    }

    #[inline]
    fn from_mins_ext(mins: u64) -> Duration {
        if mins > u64::MAX / SECS_PER_MINUTE {
            panic!("overflow in Duration::from_mins");
        }

        Duration::from_secs(mins * SECS_PER_MINUTE)
    }
}

pub trait TimestampExt {
    fn into_midnight(self) -> Self;
}

impl TimestampExt for Timestamp {
    fn into_midnight(self) -> Self {
        let micros_per_sec = 1_000_000;
        let seconds_in_a_day = 24 * 60 * 60;
        let micros_since_epoch = self.to_micros_since_unix_epoch();

        // Calculate seconds since midnight UTC
        let secs_since_epoch = micros_since_epoch / micros_per_sec;
        let secs_today = secs_since_epoch % seconds_in_a_day;

        let secs_since_epoch = secs_since_epoch - secs_today;
        Timestamp::from_micros_since_unix_epoch(secs_since_epoch * micros_per_sec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spacetimedb::Timestamp;

    #[test]
    fn test_into_midnight_basic() {
        // April 27, 2025 10:00:00 UTC (in micros)
        let current_micros = 1745748000000000;
        let current_timestamp = Timestamp::from_micros_since_unix_epoch(current_micros);

        // Expected: April 27, 2025 00:00:00 UTC (in micros)
        let expected_micros = 1745712000000000;
        let expected_timestamp = Timestamp::from_micros_since_unix_epoch(expected_micros);

        assert_eq!(current_timestamp.into_midnight(), expected_timestamp);
    }

    #[test]
    fn test_into_midnight_just_before_midnight() {
        // April 27, 2025 23:59:59.999999 UTC (in micros)
        let current_micros = 1745798399999999;
        let current_timestamp = Timestamp::from_micros_since_unix_epoch(current_micros);

        // Expected: April 27, 2025 00:00:00 UTC (in micros)
        let expected_micros = 1745712000000000;
        let expected_timestamp = Timestamp::from_micros_since_unix_epoch(expected_micros);

        assert_eq!(current_timestamp.into_midnight(), expected_timestamp);
    }

    #[test]
    fn test_into_midnight_at_midnight() {
        // April 27, 2025 00:00:00 UTC (in micros)
        let current_micros = 1745712000000000;
        let current_timestamp = Timestamp::from_micros_since_unix_epoch(current_micros);

        // Expected: April 27, 2025 00:00:00 UTC (in micros)
        // If it's exactly midnight, it should return the *next* midnight.
        let expected_micros = 1745712000000000;
        let expected_timestamp = Timestamp::from_micros_since_unix_epoch(expected_micros);

        assert_eq!(current_timestamp.into_midnight(), expected_timestamp);
    }

    #[test]
    fn test_into_midnight_just_after_midnight() {
        // April 27, 2025 00:00:01 UTC (in micros)
        let current_micros = 1745712001000000;
        let current_timestamp = Timestamp::from_micros_since_unix_epoch(current_micros);

        // Expected: April 27, 2025 00:00:00 UTC (in micros)
        let expected_micros = 1745712000000000;
        let expected_timestamp = Timestamp::from_micros_since_unix_epoch(expected_micros);

        assert_eq!(current_timestamp.into_midnight(), expected_timestamp);
    }
}
