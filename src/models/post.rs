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

/// Whether a post is a formal place review (requires rating) or a general post.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum MapkyAppPostKind {
    /// Formal rating of a place — requires the `rating` field.
    Review,
    /// Comment, question, reply, or media post — `rating` must be absent.
    #[default]
    Post,
}

/// Unified post type for reviews, questions, comments about an OSM place.
/// URI: /pub/mapky.app/posts/:post_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppPost {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[serde(default)]
    pub kind: MapkyAppPostKind,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub place: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub content: Option<String>,
    pub rating: Option<u8>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub attachments: Option<Vec<String>>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub parent: Option<String>,
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppPost {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn kind(&self) -> MapkyAppPostKind {
        self.kind.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn place(&self) -> String {
        self.place.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn parent(&self) -> Option<String> {
        self.parent.clone()
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
impl Json for MapkyAppPost {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppPost {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(
        kind: MapkyAppPostKind,
        place: String,
        content: Option<String>,
        rating: Option<u8>,
        attachments: Option<Vec<String>>,
        parent: Option<String>,
    ) -> Self {
        let post = MapkyAppPost {
            kind,
            place,
            content,
            rating,
            attachments,
            parent,
        };
        post.sanitize()
    }
}

impl TimestampId for MapkyAppPost {}

impl HasIdPath for MapkyAppPost {
    const PATH_SEGMENT: &'static str = "posts/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppPost {
    fn sanitize(self) -> Self {
        let place = sanitize_url(&self.place);
        let content = self.content.map(|c| c.trim().to_string());
        let parent = self.parent.map(|uri| sanitize_url(&uri));
        let attachments = self.attachments.map(|urls| {
            urls.into_iter().map(|u| sanitize_url(&u)).collect()
        });

        MapkyAppPost {
            kind: self.kind,
            place,
            content,
            rating: self.rating,
            attachments,
            parent,
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        // Validate place (OSM URL)
        validate_osm_url(&self.place)?;

        let has_content = self
            .content
            .as_ref()
            .is_some_and(|c| !c.trim().is_empty());
        let has_attachments = self
            .attachments
            .as_ref()
            .is_some_and(|a| !a.is_empty());

        // Kind-specific validation
        match self.kind {
            MapkyAppPostKind::Review => {
                if self.rating.is_none() {
                    return Err("Validation Error: Review must have a rating".into());
                }
            }
            MapkyAppPostKind::Post => {
                if self.rating.is_some() {
                    return Err(
                        "Validation Error: Post kind cannot have a rating — use Review".into(),
                    );
                }
                if !has_content && !has_attachments {
                    return Err(
                        "Validation Error: Post must have content or attachments".into(),
                    );
                }
            }
        }

        // Validate content length
        if let Some(ref content) = self.content {
            if content.chars().count() > MAX_CONTENT_LENGTH {
                return Err(format!(
                    "Validation Error: Content exceeds maximum length of {} characters",
                    MAX_CONTENT_LENGTH
                ));
            }
        }

        // Validate rating range (1-10)
        if let Some(rating) = self.rating {
            if !(1..=10).contains(&rating) {
                return Err(format!(
                    "Validation Error: Rating must be between 1 and 10, got {}",
                    rating
                ));
            }
        }

        // Validate parent URI
        if let Some(ref parent_uri) = self.parent {
            Url::parse(parent_uri).map_err(|_| {
                format!(
                    "Validation Error: Invalid parent URI format: {}",
                    parent_uri
                )
            })?;
        }

        // Validate attachments
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
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            Some("Hello World!".to_string()),
            None,
            None,
            None,
        );
        let post_id = post.create_id();
        assert_eq!(post_id.len(), 13);
    }

    #[test]
    fn test_new() {
        let content = "This is a test review".to_string();
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some(content.clone()),
            Some(8),
            None,
            None,
        );
        assert_eq!(post.kind, MapkyAppPostKind::Review);
        assert_eq!(post.content.unwrap(), content);
        assert_eq!(post.rating, Some(8));
        assert!(post.parent.is_none());
        assert!(post.attachments.is_none());
    }

    #[test]
    fn test_create_path() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            Some("Test".to_string()),
            None,
            None,
            None,
        );
        let post_id = post.create_id();
        let path = MapkyAppPost::create_path(&post_id);
        let prefix = format!("{}{}posts/", PUBLIC_PATH, MAPKY_PATH);
        assert!(path.starts_with(&prefix));
        assert_eq!(path.len(), prefix.len() + post_id.len());
    }

    #[test]
    fn test_sanitize() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            Some("  trimmed content  ".to_string()),
            None,
            Some(vec![
                "  pubky://user123/pub/mapky.app/files/0034A0X7NJ52G  ".to_string(),
            ]),
            Some("  pubky://user123/pub/mapky.app/posts/0034A0X7NJ52G  ".to_string()),
        );
        let sanitized = post.sanitize();
        assert_eq!(sanitized.content.unwrap(), "trimmed content");
        assert!(sanitized.parent.unwrap().starts_with("pubky://"));
        let att = sanitized.attachments.unwrap();
        assert!(att[0].starts_with("pubky://"));
    }

    #[test]
    fn test_validate_happy() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Great place!".to_string()),
            Some(8),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_rating_only() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            None,
            Some(5),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_attachments_only() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            None,
            None,
            Some(vec![
                "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".to_string(),
            ]),
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_empty_post_rejected() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            None,
            None,
            None,
            None,
        );
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have content"));
    }

    #[test]
    fn test_validate_rating_range() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Review".into()),
            Some(0),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_err());

        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Review".into()),
            Some(11),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_err());

        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Review".into()),
            Some(10),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());

        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Review".into()),
            Some(1),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_invalid_parent_uri() {
        let post = MapkyAppPost {
            kind: MapkyAppPostKind::Post,
            place: test_place(),
            content: Some("Valid content".into()),
            rating: None,
            attachments: None,
            parent: Some("invalid uri".into()),
        };
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid parent URI format"));
    }

    #[test]
    fn test_validate_non_pubky_attachment() {
        let post = MapkyAppPost {
            kind: MapkyAppPostKind::Post,
            place: test_place(),
            content: Some("Valid content".into()),
            rating: None,
            attachments: Some(vec!["https://example.com/file.jpg".into()]),
            parent: None,
        };
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("pubky://"));
    }

    #[test]
    fn test_validate_too_many_attachments() {
        let attachments: Vec<String> = (0..MAX_ATTACHMENTS + 1)
            .map(|i| format!("pubky://user123/pub/mapky.app/files/{:013}", i))
            .collect();
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            Some("Content".into()),
            None,
            Some(attachments),
            None,
        );
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Too many attachments"));
    }

    #[test]
    fn test_validate_invalid_place() {
        let post = MapkyAppPost {
            kind: MapkyAppPostKind::Post,
            place: "https://example.com/not-osm".into(),
            content: Some("Content".into()),
            rating: None,
            attachments: None,
            parent: None,
        };
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("OSM URL"));
    }

    #[test]
    fn test_try_from_valid() {
        let post_json = r#"{
            "kind": "review",
            "place": "https://www.openstreetmap.org/node/1573053883",
            "content": "Great place!",
            "rating": 8,
            "attachments": null,
            "parent": null
        }"#;

        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            "https://www.openstreetmap.org/node/1573053883".into(),
            Some("Great place!".into()),
            Some(8),
            None,
            None,
        );
        let id = post.create_id();

        let result = <MapkyAppPost as Validatable>::try_from(post_json.as_bytes(), &id);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.kind, MapkyAppPostKind::Review);
        assert_eq!(parsed.content.unwrap(), "Great place!");
        assert_eq!(parsed.rating, Some(8));
    }

    #[test]
    fn test_validate_review_requires_rating() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Nice spot".into()),
            None,
            None,
            None,
        );
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Review must have a rating"));
    }

    #[test]
    fn test_validate_post_no_rating() {
        let post = MapkyAppPost {
            kind: MapkyAppPostKind::Post,
            place: test_place(),
            content: Some("Nice spot".into()),
            rating: Some(7),
            attachments: None,
            parent: None,
        };
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Post kind cannot have a rating"));
    }

    #[test]
    fn test_validate_post_content_or_attachments() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            None,
            None,
            None,
            None,
        );
        let id = post.create_id();
        let result = post.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have content or attachments"));
    }

    #[test]
    fn test_validate_review_with_parent() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            test_place(),
            Some("Follow-up review".into()),
            Some(6),
            None,
            Some("pubky://user123/pub/mapky.app/posts/0034A0X7NJ52G".into()),
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_post_with_parent() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Post,
            test_place(),
            Some("Good point!".into()),
            None,
            None,
            Some("pubky://user123/pub/mapky.app/posts/0034A0X7NJ52G".into()),
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_place_accepts_way() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            "https://www.openstreetmap.org/way/987654321".into(),
            Some("Nice street".into()),
            Some(7),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_place_accepts_relation() {
        let post = MapkyAppPost::new(
            MapkyAppPostKind::Review,
            "https://www.openstreetmap.org/relation/111111".into(),
            Some("Great area".into()),
            Some(9),
            None,
            None,
        );
        let id = post.create_id();
        assert!(post.validate(Some(&id)).is_ok());
    }
}
