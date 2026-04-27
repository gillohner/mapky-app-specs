#[cfg(target_arch = "wasm32")]
use js_sys::Date;

use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Returns the current timestamp in microseconds since the UNIX epoch.
#[cfg(target_arch = "wasm32")]
pub fn timestamp() -> i64 {
    let ms = Date::now() as i64;
    ms * 1_000
}

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64
}

/// Trims whitespace and normalizes a URL if valid
pub fn sanitize_url(input: &str) -> String {
    let trimmed = input.trim();
    match Url::parse(trimmed) {
        Ok(parsed_url) => parsed_url.to_string(),
        Err(_) => trimmed.to_string(),
    }
}

/// Geographic bounding box in WGS84 (min/max latitude, min/max longitude).
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl BoundingBox {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
            max_lat,
            max_lon,
        }
    }
}

impl BoundingBox {
    pub fn validate(&self) -> Result<(), String> {
        if !(-90.0..=90.0).contains(&self.min_lat) || !(-90.0..=90.0).contains(&self.max_lat) {
            return Err(format!(
                "Validation Error: BoundingBox latitude out of range (-90..90): min_lat={}, max_lat={}",
                self.min_lat, self.max_lat
            ));
        }
        if !(-180.0..=180.0).contains(&self.min_lon) || !(-180.0..=180.0).contains(&self.max_lon) {
            return Err(format!(
                "Validation Error: BoundingBox longitude out of range (-180..180): min_lon={}, max_lon={}",
                self.min_lon, self.max_lon
            ));
        }
        if self.min_lat > self.max_lat {
            return Err(format!(
                "Validation Error: BoundingBox min_lat {} > max_lat {}",
                self.min_lat, self.max_lat
            ));
        }
        if self.min_lon > self.max_lon {
            return Err(format!(
                "Validation Error: BoundingBox min_lon {} > max_lon {}",
                self.min_lon, self.max_lon
            ));
        }
        Ok(())
    }
}
