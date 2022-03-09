use crate::specfun;

use wasm_bindgen::prelude::*;
// use log::info;

#[wasm_bindgen]
pub fn tpdf(x:f64, v:f64) -> f64 {
    1f64
    / (v.sqrt() * specfun::beta(0.5f64, v / 2f64))
    * (1f64 + x.powi(2) / v).powf(-(v + 1f64) / 2f64)
}

#[wasm_bindgen]
pub fn tcdf(x:f64, v:f64) -> f64 {
    let xsq = x.powi(2);

    if v < xsq {
        let p = specfun::betainc( v / (v + xsq), v/2.0, 0.5) * 0.5;
        if x>0.0 { 1. - p } else { p }
    }
    else if v >= xsq {
        let p = (1.0 - specfun::betainc( xsq / (v + xsq), 0.5, v/2.0)) * 0.5;
        if x>0.0 { 1. - p } else { p }
    }
    else {
        0.5
    }

}
