use std::f64::consts::PI;
use crate::specfun;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn normpdf(x:f64, m:f64, s:f64) -> f64 {
    1f64 / ((2f64 * PI).sqrt() * s) * (-0.5 * ((x - m) / s).powi(2)).exp()
}

fn phi(x: f64) -> f64 {
    0.5 * (1f64 + specfun::erf(x / 2f64.sqrt()))
}

#[wasm_bindgen]
pub fn normcdf(x:f64, m:f64, s:f64) -> f64 {
    phi((x - m) / s)
}

#[wasm_bindgen]
pub fn norminv(p:f64, m:f64, s:f64) -> f64 {
    -(2f64).sqrt() * specfun::erfcinv(2f64 * p) * s + m
}
