const MIN_CAR_PRODUCTION_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = if speed > 10 { 10 } else { speed };
    let rate: f64 = match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0
    };
    speed as f64 * MIN_CAR_PRODUCTION_PER_HOUR * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0_f64) as u32
}
