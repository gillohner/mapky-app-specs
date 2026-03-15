use crate::{
    common::{sanitize_tag_label, timestamp, validate_tag_label},
    constants::{INVALID_TAG_CHARS, MAX_TAG_LABEL_LENGTH, MIN_TAG_LABEL_LENGTH},
    models::osm_ref::OsmRef,
    traits::{HasIdPath, HashId, Validatable},
    MAPKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Categorized location tag with optional per-category rating.
/// URI: /pub/mapky.app/location_tags/:tag_id
/// HashId = Blake3("{osm_canonical}:{category}:{label}")
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MapkyAppLocationTag {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub place: OsmRef,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub category: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub label: String,
    pub rating: Option<u8>,
    pub created_at: i64,
}

impl MapkyAppLocationTag {
    pub fn new(
        place: OsmRef,
        category: String,
        label: String,
        rating: Option<u8>,
    ) -> Self {
        let created_at = timestamp();
        Self {
            place,
            category,
            label,
            rating,
            created_at,
        }
        .sanitize()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppLocationTag {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn category(&self) -> String {
        self.category.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn label(&self) -> String {
        self.label.clone()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for MapkyAppLocationTag {}

impl HashId for MapkyAppLocationTag {
    fn get_id_data(&self) -> String {
        format!("{}:{}:{}", self.place.canonical(), self.category, self.label)
    }
}

impl HasIdPath for MapkyAppLocationTag {
    const PATH_SEGMENT: &'static str = "location_tags/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppLocationTag {
    fn sanitize(self) -> Self {
        let category = sanitize_tag_label(&self.category);
        let label = sanitize_tag_label(&self.label);

        MapkyAppLocationTag {
            place: self.place,
            category,
            label,
            rating: self.rating,
            created_at: self.created_at,
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        self.place.validate()?;

        validate_tag_label(
            &self.category,
            MAX_TAG_LABEL_LENGTH,
            MIN_TAG_LABEL_LENGTH,
            INVALID_TAG_CHARS,
        )
        .map_err(|e| e.replace("Tag", "Category"))?;

        validate_tag_label(
            &self.label,
            MAX_TAG_LABEL_LENGTH,
            MIN_TAG_LABEL_LENGTH,
            INVALID_TAG_CHARS,
        )?;

        if let Some(rating) = self.rating {
            if !(1..=10).contains(&rating) {
                return Err(format!(
                    "Validation Error: Rating must be between 1 and 10, got {}",
                    rating
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::osm_ref::{OsmElementType, OsmRef};

    fn test_place() -> OsmRef {
        OsmRef::new(OsmElementType::Node, 1573053883)
    }

    #[test]
    fn test_create_id_deterministic() {
        let tag1 = MapkyAppLocationTag {
            place: test_place(),
            category: "safety".into(),
            label: "well-lit".into(),
            rating: Some(8),
            created_at: 1000,
        };
        let tag2 = MapkyAppLocationTag {
            place: test_place(),
            category: "safety".into(),
            label: "well-lit".into(),
            rating: Some(3), // different rating, same ID
            created_at: 2000,
        };
        assert_eq!(tag1.create_id(), tag2.create_id());
    }

    #[test]
    fn test_different_label_different_id() {
        let tag1 = MapkyAppLocationTag {
            place: test_place(),
            category: "safety".into(),
            label: "well-lit".into(),
            rating: None,
            created_at: 1000,
        };
        let tag2 = MapkyAppLocationTag {
            place: test_place(),
            category: "safety".into(),
            label: "dark".into(),
            rating: None,
            created_at: 1000,
        };
        assert_ne!(tag1.create_id(), tag2.create_id());
    }

    #[test]
    fn test_new() {
        let tag = MapkyAppLocationTag::new(
            test_place(),
            "Safety".into(),
            "Well-Lit".into(),
            Some(8),
        );
        assert_eq!(tag.category, "safety");
        assert_eq!(tag.label, "well-lit");
        assert_eq!(tag.rating, Some(8));
    }

    #[test]
    fn test_create_path() {
        let tag = MapkyAppLocationTag::new(
            test_place(),
            "safety".into(),
            "well-lit".into(),
            None,
        );
        let id = tag.create_id();
        let path = MapkyAppLocationTag::create_path(&id);
        let prefix = format!("{}{}location_tags/", PUBLIC_PATH, MAPKY_PATH);
        assert!(path.starts_with(&prefix));
    }

    #[test]
    fn test_validate_happy() {
        let tag = MapkyAppLocationTag::new(
            test_place(),
            "safety".into(),
            "well-lit".into(),
            Some(8),
        );
        let id = tag.create_id();
        assert!(tag.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_invalid_rating() {
        let tag = MapkyAppLocationTag::new(
            test_place(),
            "safety".into(),
            "well-lit".into(),
            Some(11),
        );
        let id = tag.create_id();
        assert!(tag.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_empty_label() {
        let tag = MapkyAppLocationTag {
            place: test_place(),
            category: "safety".into(),
            label: "".into(),
            rating: None,
            created_at: 1000,
        };
        let id = tag.create_id();
        assert!(tag.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_invalid_place() {
        let tag = MapkyAppLocationTag::new(
            OsmRef::new(OsmElementType::Node, 0),
            "safety".into(),
            "well-lit".into(),
            None,
        );
        let id = tag.create_id();
        assert!(tag.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_try_from_valid() {
        let tag_json = r#"{
            "place": {"osm_type": "node", "osm_id": 1573053883},
            "category": "safety",
            "label": "well-lit",
            "rating": 8,
            "created_at": 1627849723000
        }"#;

        let tag = MapkyAppLocationTag::new(
            test_place(),
            "safety".into(),
            "well-lit".into(),
            Some(8),
        );
        let id = tag.create_id();

        let result = <MapkyAppLocationTag as Validatable>::try_from(tag_json.as_bytes(), &id);
        assert!(result.is_ok());
    }
}
