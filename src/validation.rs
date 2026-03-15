use url::Url;

/// Validates geographic coordinates
pub fn validate_coordinates(lat: f64, lon: f64) -> Result<(), String> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(format!(
            "Validation Error: Latitude {} out of range (-90 to 90)",
            lat
        ));
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(format!(
            "Validation Error: Longitude {} out of range (-180 to 180)",
            lon
        ));
    }
    Ok(())
}

/// Validates a compass heading (0-360 degrees)
pub fn validate_heading(heading: f64) -> Result<(), String> {
    if !(0.0..=360.0).contains(&heading) {
        return Err(format!(
            "Validation Error: Heading {} out of range (0 to 360)",
            heading
        ));
    }
    Ok(())
}

/// Validates a pubky:// URI
pub fn validate_pubky_uri(uri: &str) -> Result<(), String> {
    let parsed = Url::parse(uri)
        .map_err(|_| format!("Validation Error: Invalid URI format: {}", uri))?;
    if parsed.scheme() != "pubky" {
        return Err(format!(
            "Validation Error: URI must use pubky:// protocol: {}",
            uri
        ));
    }
    Ok(())
}
