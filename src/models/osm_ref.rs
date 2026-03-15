use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// OSM element type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[serde(rename_all = "lowercase")]
pub enum OsmElementType {
    Node,
    Way,
    Relation,
}

impl fmt::Display for OsmElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsmElementType::Node => write!(f, "node"),
            OsmElementType::Way => write!(f, "way"),
            OsmElementType::Relation => write!(f, "relation"),
        }
    }
}

/// Canonical reference to an OSM element. Embedded in other models, not stored standalone.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OsmRef {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub osm_type: OsmElementType,
    pub osm_id: i64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl OsmRef {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(osm_type: OsmElementType, osm_id: i64) -> Self {
        Self { osm_type, osm_id }
    }

    /// Returns canonical string: "node/123456789"
    pub fn canonical(&self) -> String {
        format!("{}/{}", self.osm_type, self.osm_id)
    }

    /// Returns OSM URL: "https://www.openstreetmap.org/node/123456789"
    pub fn osm_url(&self) -> String {
        format!("https://www.openstreetmap.org/{}/{}", self.osm_type, self.osm_id)
    }
}

impl OsmRef {
    /// Parse from canonical string "node/123456789"
    pub fn from_canonical(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.splitn(2, '/').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid canonical format: {}", s));
        }

        let osm_type = match parts[0] {
            "node" => OsmElementType::Node,
            "way" => OsmElementType::Way,
            "relation" => OsmElementType::Relation,
            _ => return Err(format!("Invalid OSM element type: {}", parts[0])),
        };

        let osm_id: i64 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid OSM ID: {}", parts[1]))?;

        Ok(Self { osm_type, osm_id })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.osm_id <= 0 {
            return Err(format!(
                "Validation Error: OSM ID must be positive, got {}",
                self.osm_id
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical() {
        let osm_ref = OsmRef::new(OsmElementType::Node, 1573053883);
        assert_eq!(osm_ref.canonical(), "node/1573053883");
    }

    #[test]
    fn test_osm_url() {
        let osm_ref = OsmRef::new(OsmElementType::Way, 987654321);
        assert_eq!(
            osm_ref.osm_url(),
            "https://www.openstreetmap.org/way/987654321"
        );
    }

    #[test]
    fn test_from_canonical() {
        let osm_ref = OsmRef::from_canonical("relation/111111").unwrap();
        assert_eq!(osm_ref.osm_type, OsmElementType::Relation);
        assert_eq!(osm_ref.osm_id, 111111);
    }

    #[test]
    fn test_from_canonical_roundtrip() {
        let original = OsmRef::new(OsmElementType::Node, 42);
        let canonical = original.canonical();
        let parsed = OsmRef::from_canonical(&canonical).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_from_canonical_invalid_type() {
        let result = OsmRef::from_canonical("invalid/123");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_canonical_invalid_format() {
        let result = OsmRef::from_canonical("node123");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_positive_id() {
        let osm_ref = OsmRef::new(OsmElementType::Node, 1);
        assert!(osm_ref.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_id() {
        let osm_ref = OsmRef::new(OsmElementType::Node, 0);
        assert!(osm_ref.validate().is_err());
    }

    #[test]
    fn test_validate_negative_id() {
        let osm_ref = OsmRef::new(OsmElementType::Node, -1);
        assert!(osm_ref.validate().is_err());
    }

    #[test]
    fn test_serde_roundtrip() {
        let osm_ref = OsmRef::new(OsmElementType::Way, 42);
        let json = serde_json::to_string(&osm_ref).unwrap();
        let parsed: OsmRef = serde_json::from_str(&json).unwrap();
        assert_eq!(osm_ref, parsed);
        // Verify serde rename_all = "lowercase"
        assert!(json.contains("\"way\""));
    }
}
