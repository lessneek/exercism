use std::time::Duration;

use time::PrimitiveDateTime as DateTime;

const AFTER_SECONDS: u64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::new(AFTER_SECONDS, 0)
}
