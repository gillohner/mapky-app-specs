use crate::{
    common::{sanitize_url, timestamp},
    constants::{
        MAX_ATTACHMENT_URL_LENGTH, MAX_INCIDENT_ATTACHMENTS, MAX_INCIDENT_DESCRIPTION_LENGTH,
    },
    traits::{HasIdPath, TimestampId, Validatable},
    validation::{validate_coordinates, validate_heading, validate_pubky_uri},
    MAPKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum IncidentType {
    Accident,
    Hazard,
    RoadClosure,
    Police,
    Flooding,
    IceSnow,
    PoorVisibility,
    Danger,
    #[default]
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum IncidentSeverity {
    Low,
    #[default]
    Medium,
    High,
}

/// Waze-style crowdsourced incident report, coordinate-anchored.
/// URI: /pub/mapky.app/incidents/:incident_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppIncident {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub incident_type: IncidentType,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub severity: IncidentSeverity,
    pub lat: f64,
    pub lon: f64,
    pub heading: Option<f64>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub attachments: Option<Vec<String>>,
    pub expires_at: Option<i64>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppIncident {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(
        incident_type: IncidentType,
        severity: IncidentSeverity,
        lat: f64,
        lon: f64,
    ) -> Self {
        MapkyAppIncident {
            incident_type,
            severity,
            lat,
            lon,
            heading: None,
            description: None,
            attachments: None,
            expires_at: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppIncident {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for MapkyAppIncident {}

impl TimestampId for MapkyAppIncident {}

impl HasIdPath for MapkyAppIncident {
    const PATH_SEGMENT: &'static str = "incidents/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppIncident {
    fn sanitize(self) -> Self {
        let description = self.description.map(|d| d.trim().to_string());
        let attachments = self
            .attachments
            .map(|urls| urls.into_iter().map(|u| sanitize_url(&u)).collect());

        MapkyAppIncident {
            description,
            attachments,
            ..self
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        validate_coordinates(self.lat, self.lon)?;

        if let Some(heading) = self.heading {
            validate_heading(heading)?;
        }

        if let Some(ref desc) = self.description {
            if desc.chars().count() > MAX_INCIDENT_DESCRIPTION_LENGTH {
                return Err(format!(
                    "Validation Error: Description exceeds maximum length of {} characters",
                    MAX_INCIDENT_DESCRIPTION_LENGTH
                ));
            }
        }

        // Validate expires_at must be in the future if present
        if let Some(expires_at) = self.expires_at {
            let now = timestamp();
            if expires_at <= now {
                return Err("Validation Error: expires_at must be in the future".into());
            }
        }

        if let Some(ref attachments) = self.attachments {
            if attachments.len() > MAX_INCIDENT_ATTACHMENTS {
                return Err(format!(
                    "Validation Error: Too many attachments (max: {})",
                    MAX_INCIDENT_ATTACHMENTS
                ));
            }
            for (i, url) in attachments.iter().enumerate() {
                if url.chars().count() > MAX_ATTACHMENT_URL_LENGTH {
                    return Err(format!(
                        "Validation Error: Attachment URL at index {} exceeds maximum length",
                        i
                    ));
                }
                validate_pubky_uri(url)
                    .map_err(|e| format!("Validation Error: Attachment at index {}: {}", i, e))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_id() {
        let incident = MapkyAppIncident::new(
            IncidentType::Accident,
            IncidentSeverity::High,
            47.3769,
            8.5417,
        );
        let id = incident.create_id();
        assert_eq!(id.len(), 13);
    }

    #[test]
    fn test_create_path() {
        let incident = MapkyAppIncident::new(IncidentType::Hazard, IncidentSeverity::Low, 0.0, 0.0);
        let id = incident.create_id();
        let path = MapkyAppIncident::create_path(&id);
        assert!(path.starts_with("/pub/mapky.app/incidents/"));
    }

    #[test]
    fn test_validate_happy() {
        let incident = MapkyAppIncident::new(
            IncidentType::Flooding,
            IncidentSeverity::Medium,
            47.3769,
            8.5417,
        );
        let id = incident.create_id();
        assert!(incident.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_invalid_lat() {
        let incident =
            MapkyAppIncident::new(IncidentType::Accident, IncidentSeverity::High, 91.0, 0.0);
        let id = incident.create_id();
        assert!(incident.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_invalid_lon() {
        let incident =
            MapkyAppIncident::new(IncidentType::Accident, IncidentSeverity::High, 0.0, 181.0);
        let id = incident.create_id();
        assert!(incident.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_invalid_heading() {
        let mut incident =
            MapkyAppIncident::new(IncidentType::Accident, IncidentSeverity::High, 0.0, 0.0);
        incident.heading = Some(361.0);
        let id = incident.create_id();
        assert!(incident.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_too_many_attachments() {
        let mut incident = MapkyAppIncident::new(
            IncidentType::Accident,
            IncidentSeverity::High,
            47.3769,
            8.5417,
        );
        incident.attachments = Some(
            (0..6)
                .map(|i| format!("pubky://user123/pub/mapky.app/files/{:013}", i))
                .collect(),
        );
        let id = incident.create_id();
        let result = incident.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Too many attachments"));
    }

    #[test]
    fn test_validate_five_attachments_ok() {
        let mut incident = MapkyAppIncident::new(
            IncidentType::Accident,
            IncidentSeverity::High,
            47.3769,
            8.5417,
        );
        incident.attachments = Some(
            (0..5)
                .map(|i| format!("pubky://user123/pub/mapky.app/files/{:013}", i))
                .collect(),
        );
        let id = incident.create_id();
        assert!(incident.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_expires_at_in_past() {
        let mut incident =
            MapkyAppIncident::new(IncidentType::Hazard, IncidentSeverity::Low, 0.0, 0.0);
        incident.expires_at = Some(1_000_000); // way in the past (microseconds)
        let id = incident.create_id();
        let result = incident.validate(Some(&id));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("expires_at must be in the future"));
    }

    #[test]
    fn test_validate_expires_at_in_future() {
        let mut incident =
            MapkyAppIncident::new(IncidentType::Hazard, IncidentSeverity::Low, 0.0, 0.0);
        // Set to far in the future (year ~2100 in microseconds)
        incident.expires_at = Some(4_102_444_800_000_000);
        let id = incident.create_id();
        assert!(incident.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_all_incident_types() {
        let types = vec![
            IncidentType::Accident,
            IncidentType::Hazard,
            IncidentType::RoadClosure,
            IncidentType::Police,
            IncidentType::Flooding,
            IncidentType::IceSnow,
            IncidentType::PoorVisibility,
            IncidentType::Danger,
            IncidentType::Other,
        ];
        for t in types {
            let incident = MapkyAppIncident::new(t, IncidentSeverity::Low, 0.0, 0.0);
            let id = incident.create_id();
            assert!(incident.validate(Some(&id)).is_ok());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let incident = MapkyAppIncident::new(
            IncidentType::RoadClosure,
            IncidentSeverity::High,
            47.3769,
            8.5417,
        );
        let json = serde_json::to_string(&incident).unwrap();
        assert!(json.contains("\"road_closure\""));
        assert!(json.contains("\"high\""));
        let parsed: MapkyAppIncident = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.incident_type, IncidentType::RoadClosure);
    }

    #[test]
    fn test_try_from_valid() {
        let json = r#"{
            "incident_type": "accident",
            "severity": "high",
            "lat": 47.3769,
            "lon": 8.5417,
            "heading": null,
            "description": "Multi-car pileup",
            "attachments": null,
            "expires_at": null
        }"#;
        let incident = MapkyAppIncident::new(
            IncidentType::Accident,
            IncidentSeverity::High,
            47.3769,
            8.5417,
        );
        let id = incident.create_id();
        let result = <MapkyAppIncident as Validatable>::try_from(json.as_bytes(), &id);
        assert!(result.is_ok());
    }
}
