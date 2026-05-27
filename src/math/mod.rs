pub fn ease_out_circ(x: f64) -> f64 {
    (1.0 - (x - 1.0).powi(2)).sqrt()
}

pub fn reciprocal_decay(x: f64) -> f64 {
    10.0 / (1.0 + x / 50.0)
}
