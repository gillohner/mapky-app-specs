use crate::{
    common::sanitize_url,
    constants::{MAX_COLLECTION_ITEMS, MAX_COLLECTION_NAME_LENGTH, MAX_DESCRIPTION_LENGTH},
    traits::{HasIdPath, TimestampId, Validatable},
    validation::validate_osm_url,
    MAPKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Named list of places.
/// URI: /pub/mapky.app/collections/:collection_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppCollection {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub name: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub items: Vec<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub image_uri: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppCollection {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(
        name: String,
        description: Option<String>,
        items: Vec<String>,
        image_uri: Option<String>,
    ) -> Self {
        let collection = MapkyAppCollection {
            name,
            description,
            items,
            image_uri,
        };
        collection.sanitize()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppCollection {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for MapkyAppCollection {}

impl TimestampId for MapkyAppCollection {}

impl HasIdPath for MapkyAppCollection {
    const PATH_SEGMENT: &'static str = "collections/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppCollection {
    fn sanitize(self) -> Self {
        let name = self.name.trim().to_string();
        let description = self.description.map(|d| d.trim().to_string());
        let image_uri = self.image_uri.map(|u| sanitize_url(&u));
        let items = self.items.into_iter().map(|u| sanitize_url(&u)).collect();

        MapkyAppCollection {
            name,
            description,
            items,
            image_uri,
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        // Validate name
        if self.name.trim().is_empty() {
            return Err("Validation Error: Collection name cannot be empty".into());
        }
        if self.name.chars().count() > MAX_COLLECTION_NAME_LENGTH {
            return Err(format!(
                "Validation Error: Collection name exceeds maximum length of {} characters",
                MAX_COLLECTION_NAME_LENGTH
            ));
        }

        // Validate description
        if let Some(ref desc) = self.description {
            if desc.chars().count() > MAX_DESCRIPTION_LENGTH {
                return Err(format!(
                    "Validation Error: Description exceeds maximum length of {} characters",
                    MAX_DESCRIPTION_LENGTH
                ));
            }
        }

        // Validate items (0–500 allowed; empty collection is valid per spec)
        if self.items.len() > MAX_COLLECTION_ITEMS {
            return Err(format!(
                "Validation Error: Collection exceeds maximum of {} items",
                MAX_COLLECTION_ITEMS
            ));
        }

        // Validate each item and check for duplicates
        let mut seen = HashSet::new();
        for (i, item) in self.items.iter().enumerate() {
            validate_osm_url(item).map_err(|e| {
                format!("Validation Error: Item at index {}: {}", i, e)
            })?;
            if !seen.insert(item.clone()) {
                return Err(format!(
                    "Validation Error: Duplicate item in collection: {}",
                    item
                ));
            }
        }

        // Validate image URI
        if let Some(ref uri) = self.image_uri {
            url::Url::parse(uri)
                .map_err(|_| format!("Validation Error: Invalid image URI: {}", uri))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_items() -> Vec<String> {
        vec![
            "https://www.openstreetmap.org/node/1".into(),
            "https://www.openstreetmap.org/node/2".into(),
        ]
    }

    #[test]
    fn test_create_id() {
        let c = MapkyAppCollection::new("My List".into(), None, test_items(), None);
        let id = c.create_id();
        assert_eq!(id.len(), 13);
    }

    #[test]
    fn test_create_path() {
        let c = MapkyAppCollection::new("My List".into(), None, test_items(), None);
        let id = c.create_id();
        let path = MapkyAppCollection::create_path(&id);
        assert!(path.starts_with("/pub/mapky.app/collections/"));
    }

    #[test]
    fn test_validate_happy() {
        let c = MapkyAppCollection::new("My List".into(), None, test_items(), None);
        let id = c.create_id();
        assert!(c.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let c = MapkyAppCollection::new("".into(), None, test_items(), None);
        let id = c.create_id();
        assert!(c.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_empty_items_allowed() {
        let c = MapkyAppCollection::new("List".into(), None, vec![], None);
        let id = c.create_id();
        assert!(c.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_duplicate_items() {
        let items = vec![
            "https://www.openstreetmap.org/node/1".into(),
            "https://www.openstreetmap.org/node/1".into(),
        ];
        let c = MapkyAppCollection::new("List".into(), None, items, None);
        let id = c.create_id();
        let result = c.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate"));
    }

    #[test]
    fn test_validate_name_too_long() {
        let c = MapkyAppCollection::new(
            "a".repeat(MAX_COLLECTION_NAME_LENGTH + 1),
            None,
            test_items(),
            None,
        );
        let id = c.create_id();
        assert!(c.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_invalid_item() {
        let items = vec!["https://example.com/not-osm".into()];
        let c = MapkyAppCollection::new("List".into(), None, items, None);
        let id = c.create_id();
        let result = c.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("OSM URL"));
    }

    #[test]
    fn test_try_from_valid() {
        let json = r#"{
            "name": "My Favorite Spots",
            "description": null,
            "items": [
                "https://www.openstreetmap.org/node/1",
                "https://www.openstreetmap.org/node/2"
            ],
            "image_uri": null
        }"#;
        let c = MapkyAppCollection::new("My Favorite Spots".into(), None, test_items(), None);
        let id = c.create_id();
        let result = <MapkyAppCollection as Validatable>::try_from(json.as_bytes(), &id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mixed_osm_types() {
        let items = vec![
            "https://www.openstreetmap.org/node/1".into(),
            "https://www.openstreetmap.org/way/2".into(),
            "https://www.openstreetmap.org/relation/3".into(),
        ];
        let c = MapkyAppCollection::new("Mixed".into(), None, items, None);
        let id = c.create_id();
        assert!(c.validate(Some(&id)).is_ok());
    }
}
