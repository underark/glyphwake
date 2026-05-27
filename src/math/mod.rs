pub fn ease_out_circ(x: f64) -> f64 {
    (1.0 - (x - 1.0).powi(2)).sqrt()
}

pub fn reciprocal_decay(x: f64, max: f64, scaling: f64) -> f64 {
    max / (1.0 + x / scaling)
}

pub fn quadratic(x: f64) -> f64 {
    x.powi(2)
}
