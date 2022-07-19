// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
const BASE_CAR_RATE: u32 = 221;
const MIN_IN_HR: u32 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production_per_hour: u32 = BASE_CAR_RATE * (speed as u32);
    let fault_factor = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9|10 => 0.77,
        _ => panic!("Production rate must be between 0 and 10, got {}.", speed),
    };
    production_per_hour as f64 * fault_factor
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / MIN_IN_HR as f64).floor() as u32
}
