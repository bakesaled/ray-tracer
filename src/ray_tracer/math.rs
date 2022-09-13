use rand::Rng;

/// Generates a random number between 0 and 1.
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// Generates a random number within a specified range.
pub fn random_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
