// uses puruspe
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn beta(z:f64, w:f64) -> f64 {
    puruspe::beta(z, w)
}

#[wasm_bindgen]
pub fn betainc(x:f64, z:f64, w:f64) -> f64 {
    puruspe::betai(z, w, x)
}


#[wasm_bindgen]
pub fn betaincinv(y:f64, z:f64, w:f64) -> f64 {
    puruspe::invbetai(y, z, w)
}


#[wasm_bindgen]
pub fn erf(x:f64) -> f64 {
    puruspe::erf(x)
}

#[wasm_bindgen]
pub fn erfc(x:f64) -> f64 {
    puruspe::erfc(x)
}

#[wasm_bindgen]
pub fn erfinv(x:f64) -> f64 {
    puruspe::inverf(x)
}

#[wasm_bindgen]
pub fn erfcinv(p:f64) -> f64 {
    puruspe::inverfc(p)
}


#[wasm_bindgen]
pub fn gamma(z:f64) -> f64 {
    puruspe::gamma(z)
}

#[wasm_bindgen]
pub fn gammaln(x:f64) -> f64 {
    puruspe::ln_gamma(x)
}


#[wasm_bindgen]
pub fn gammainc(x:f64, a:f64) -> f64 {
    puruspe::gammp(a, x)
}

#[wasm_bindgen]
pub fn gammaincinv(y:f64, a:f64) -> f64 {
    puruspe::invgammp(y, a)
}