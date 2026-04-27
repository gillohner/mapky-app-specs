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

/// Validates that a pubky:// URI points at a `/pub/mapky.app/sequences/{id}` resource.
pub fn validate_sequence_uri(uri: &str) -> Result<(), String> {
    validate_pubky_uri(uri)?;
    let parsed = Url::parse(uri).unwrap();
    if !parsed.path().contains("/pub/mapky.app/sequences/") {
        return Err(format!(
            "Validation Error: sequence_uri must reference a /pub/mapky.app/sequences/ resource: {}",
            uri
        ));
    }
    Ok(())
}

/// Validates a microsecond-precision UNIX timestamp. Rejects non-positive values and
/// anything more than 1 day in the future (clock-skew tolerance).
pub fn validate_timestamp_microseconds(ts_us: i64, field: &str) -> Result<(), String> {
    if ts_us <= 0 {
        return Err(format!(
            "Validation Error: {field} must be a positive UNIX timestamp in microseconds, got {ts_us}"
        ));
    }
    let now_us = crate::common::timestamp();
    let max_future_us = now_us + 86_400_000_000; // +1 day
    if ts_us > max_future_us {
        return Err(format!(
            "Validation Error: {field} {ts_us} is more than 1 day in the future"
        ));
    }
    Ok(())
}

/// OSM URL host
const OSM_HOST: &str = "www.openstreetmap.org";

/// Valid OSM element types in URLs
const OSM_ELEMENT_TYPES: [&str; 3] = ["node", "way", "relation"];

/// Validates an OSM URL: https://www.openstreetmap.org/{node|way|relation}/{positive_id}
pub fn validate_osm_url(url: &str) -> Result<(), String> {
    let parsed = Url::parse(url)
        .map_err(|e| format!("Validation Error: Invalid OSM URL: {}", e))?;
    if parsed.scheme() != "https" || parsed.host_str() != Some(OSM_HOST) {
        return Err(
            "Validation Error: OSM URL must be https://www.openstreetmap.org/...".into(),
        );
    }
    let segments: Vec<&str> = parsed.path().trim_start_matches('/').split('/').collect();
    if segments.len() != 2 {
        return Err("Validation Error: OSM URL path must be /{type}/{id}".into());
    }
    if !OSM_ELEMENT_TYPES.contains(&segments[0]) {
        return Err(format!(
            "Validation Error: Invalid OSM element type: {}",
            segments[0]
        ));
    }
    let id: i64 = segments[1]
        .parse()
        .map_err(|_| "Validation Error: OSM ID must be a positive integer".to_string())?;
    if id <= 0 {
        return Err(format!(
            "Validation Error: OSM ID must be positive, got {}",
            id
        ));
    }
    Ok(())
}

/// Validates that an OSM URL refers to a Way element
pub fn validate_osm_way_url(url: &str) -> Result<(), String> {
    validate_osm_url(url)?;
    // Safe to unwrap: already validated above
    let parsed = Url::parse(url).unwrap();
    if !parsed.path().starts_with("/way/") {
        return Err("Validation Error: Expected an OSM Way URL".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_osm_url_node() {
        assert!(validate_osm_url("https://www.openstreetmap.org/node/1573053883").is_ok());
    }

    #[test]
    fn test_validate_osm_url_way() {
        assert!(validate_osm_url("https://www.openstreetmap.org/way/987654321").is_ok());
    }

    #[test]
    fn test_validate_osm_url_relation() {
        assert!(validate_osm_url("https://www.openstreetmap.org/relation/111111").is_ok());
    }

    #[test]
    fn test_validate_osm_url_wrong_host() {
        assert!(validate_osm_url("https://example.com/node/123").is_err());
    }

    #[test]
    fn test_validate_osm_url_wrong_scheme() {
        assert!(validate_osm_url("http://www.openstreetmap.org/node/123").is_err());
    }

    #[test]
    fn test_validate_osm_url_invalid_type() {
        assert!(validate_osm_url("https://www.openstreetmap.org/changeset/123").is_err());
    }

    #[test]
    fn test_validate_osm_url_zero_id() {
        assert!(validate_osm_url("https://www.openstreetmap.org/node/0").is_err());
    }

    #[test]
    fn test_validate_osm_url_negative_id() {
        assert!(validate_osm_url("https://www.openstreetmap.org/node/-1").is_err());
    }

    #[test]
    fn test_validate_osm_url_non_numeric_id() {
        assert!(validate_osm_url("https://www.openstreetmap.org/node/abc").is_err());
    }

    #[test]
    fn test_validate_osm_url_extra_path() {
        assert!(validate_osm_url("https://www.openstreetmap.org/node/123/extra").is_err());
    }

    #[test]
    fn test_validate_osm_way_url_valid() {
        assert!(validate_osm_way_url("https://www.openstreetmap.org/way/456").is_ok());
    }

    #[test]
    fn test_validate_osm_way_url_rejects_node() {
        let result = validate_osm_way_url("https://www.openstreetmap.org/node/456");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Way URL"));
    }
}
