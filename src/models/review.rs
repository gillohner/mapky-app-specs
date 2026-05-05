use crate::{
    common::sanitize_url,
    constants::{MAX_ATTACHMENTS, MAX_ATTACHMENT_URL_LENGTH, MAX_CONTENT_LENGTH},
    traits::{HasIdPath, TimestampId, Validatable},
    validation::validate_osm_url,
    MAPKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Formal rating of an OSM place. Always anchored to a place; never a reply.
/// Generic comments and threaded replies live in `PubkyAppPost` blobs stored
/// at `/pub/mapky.app/posts/{id}` and are handled by the plugin's post handler.
///
/// URI: /pub/mapky.app/reviews/:review_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppReview {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub place: String,
    pub rating: u8,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub content: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub attachments: Option<Vec<String>>,
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppReview {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn place(&self) -> String {
        self.place.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn attachments(&self) -> Option<Vec<String>> {
        self.attachments.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for MapkyAppReview {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppReview {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(
        place: String,
        rating: u8,
        content: Option<String>,
        attachments: Option<Vec<String>>,
    ) -> Self {
        let review = MapkyAppReview {
            place,
            rating,
            content,
            attachments,
        };
        review.sanitize()
    }
}

impl TimestampId for MapkyAppReview {}

impl HasIdPath for MapkyAppReview {
    const PATH_SEGMENT: &'static str = "reviews/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppReview {
    fn sanitize(self) -> Self {
        let place = sanitize_url(&self.place);
        let content = self.content.map(|c| c.trim().to_string());
        let attachments = self
            .attachments
            .map(|urls| urls.into_iter().map(|u| sanitize_url(&u)).collect());

        MapkyAppReview {
            place,
            rating: self.rating,
            content,
            attachments,
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        validate_osm_url(&self.place)?;

        if !(1..=10).contains(&self.rating) {
            return Err(format!(
                "Validation Error: Rating must be between 1 and 10, got {}",
                self.rating
            ));
        }

        if let Some(ref content) = self.content {
            if content.chars().count() > MAX_CONTENT_LENGTH {
                return Err(format!(
                    "Validation Error: Content exceeds maximum length of {} characters",
                    MAX_CONTENT_LENGTH
                ));
            }
        }

        if let Some(ref attachments) = self.attachments {
            if attachments.len() > MAX_ATTACHMENTS {
                return Err(format!(
                    "Validation Error: Too many attachments (max: {})",
                    MAX_ATTACHMENTS
                ));
            }

            for (index, url) in attachments.iter().enumerate() {
                if url.trim().is_empty() {
                    return Err(format!(
                        "Validation Error: Attachment URL at index {} cannot be empty",
                        index
                    ));
                }
                if url.chars().count() > MAX_ATTACHMENT_URL_LENGTH {
                    return Err(format!(
                        "Validation Error: Attachment URL at index {} exceeds maximum length (max: {} characters)",
                        index, MAX_ATTACHMENT_URL_LENGTH
                    ));
                }
                let parsed_url = Url::parse(url).map_err(|_| {
                    format!(
                        "Validation Error: Invalid attachment URL format at index {}",
                        index
                    )
                })?;
                if parsed_url.scheme() != "pubky" {
                    return Err(format!(
                        "Validation Error: Attachment URL at index {} must use pubky:// protocol",
                        index
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Validatable;

    fn test_place() -> String {
        "https://www.openstreetmap.org/node/1573053883".to_string()
    }

    #[test]
    fn test_create_id() {
        let review = MapkyAppReview::new(test_place(), 8, Some("Hello".into()), None);
        let id = review.create_id();
        assert_eq!(id.len(), 13);
    }

    #[test]
    fn test_new() {
        let review = MapkyAppReview::new(test_place(), 8, Some("This is a test review".into()), None);
        assert_eq!(review.rating, 8);
        assert_eq!(review.content.unwrap(), "This is a test review");
        assert!(review.attachments.is_none());
    }

    #[test]
    fn test_create_path() {
        let review = MapkyAppReview::new(test_place(), 5, None, None);
        let id = review.create_id();
        let path = MapkyAppReview::create_path(&id);
        let prefix = format!("{}{}reviews/", PUBLIC_PATH, MAPKY_PATH);
        assert!(path.starts_with(&prefix));
        assert_eq!(path.len(), prefix.len() + id.len());
    }

    #[test]
    fn test_sanitize() {
        let review = MapkyAppReview::new(
            test_place(),
            7,
            Some("  trimmed content  ".into()),
            Some(vec![
                "  pubky://user123/pub/mapky.app/files/0034A0X7NJ52G  ".into(),
            ]),
        );
        let sanitized = review.sanitize();
        assert_eq!(sanitized.content.unwrap(), "trimmed content");
        let att = sanitized.attachments.unwrap();
        assert!(att[0].starts_with("pubky://"));
    }

    #[test]
    fn test_validate_happy() {
        let review = MapkyAppReview::new(test_place(), 8, Some("Great!".into()), None);
        let id = review.create_id();
        assert!(review.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_rating_only() {
        let review = MapkyAppReview::new(test_place(), 5, None, None);
        let id = review.create_id();
        assert!(review.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_rating_range_low() {
        let review = MapkyAppReview::new(test_place(), 0, Some("Bad".into()), None);
        let id = review.create_id();
        assert!(review.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_rating_range_high() {
        let review = MapkyAppReview::new(test_place(), 11, Some("Too high".into()), None);
        let id = review.create_id();
        assert!(review.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_rating_boundaries() {
        for r in [1u8, 10] {
            let review = MapkyAppReview::new(test_place(), r, Some("ok".into()), None);
            let id = review.create_id();
            assert!(review.validate(Some(&id)).is_ok());
        }
    }

    #[test]
    fn test_validate_non_pubky_attachment() {
        let review = MapkyAppReview {
            place: test_place(),
            rating: 6,
            content: Some("ok".into()),
            attachments: Some(vec!["https://example.com/file.jpg".into()]),
        };
        let id = review.create_id();
        let result = review.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("pubky://"));
    }

    #[test]
    fn test_validate_too_many_attachments() {
        let attachments: Vec<String> = (0..MAX_ATTACHMENTS + 1)
            .map(|i| format!("pubky://user123/pub/mapky.app/files/{:013}", i))
            .collect();
        let review = MapkyAppReview::new(test_place(), 7, None, Some(attachments));
        let id = review.create_id();
        let result = review.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Too many attachments"));
    }

    #[test]
    fn test_validate_invalid_place() {
        let review = MapkyAppReview {
            place: "https://example.com/not-osm".into(),
            rating: 7,
            content: Some("Content".into()),
            attachments: None,
        };
        let id = review.create_id();
        let result = review.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("OSM URL"));
    }

    #[test]
    fn test_try_from_valid() {
        let json = r#"{
            "place": "https://www.openstreetmap.org/node/1573053883",
            "rating": 8,
            "content": "Great place!",
            "attachments": null
        }"#;

        let id = MapkyAppReview::new(test_place(), 8, Some("Great place!".into()), None).create_id();
        let parsed =
            <MapkyAppReview as Validatable>::try_from(json.as_bytes(), &id).unwrap();
        assert_eq!(parsed.rating, 8);
        assert_eq!(parsed.content.unwrap(), "Great place!");
    }

    #[test]
    fn test_place_accepts_way() {
        let review = MapkyAppReview::new(
            "https://www.openstreetmap.org/way/987654321".into(),
            7,
            Some("Nice street".into()),
            None,
        );
        let id = review.create_id();
        assert!(review.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_place_accepts_relation() {
        let review = MapkyAppReview::new(
            "https://www.openstreetmap.org/relation/111111".into(),
            9,
            Some("Great area".into()),
            None,
        );
        let id = review.create_id();
        assert!(review.validate(Some(&id)).is_ok());
    }
}
