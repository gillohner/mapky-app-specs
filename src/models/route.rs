use crate::{
    common::sanitize_url,
    constants::{
        MAX_ROUTE_CONTROL_POINTS, MAX_ROUTE_COSTING_LENGTH, MAX_ROUTE_DESCRIPTION_LENGTH,
        MAX_ROUTE_ENGINE_LENGTH, MAX_ROUTE_INSTRUCTION_LENGTH, MAX_ROUTE_NAME_LENGTH,
        MAX_ROUTE_POLYLINE_LENGTH, MAX_ROUTE_WAYPOINTS, MAX_WAYPOINT_NAME_LENGTH, MIN_WAYPOINTS,
    },
    traits::{HasIdPath, TimestampId, Validatable},
    validation::{validate_coordinates, validate_osm_way_url},
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
pub enum RouteActivityType {
    #[default]
    Hiking,
    Cycling,
    Running,
    Walking,
    Driving,
    Skiing,
    Other,
}

/// A geographic waypoint along a route
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub lat: f64,
    pub lon: f64,
    pub ele: Option<f64>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub name: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Waypoint {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(lat: f64, lon: f64, ele: Option<f64>) -> Self {
        Self {
            lat,
            lon,
            ele,
            name: None,
        }
    }
}

/// A step in a route with a navigation instruction
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteStep {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub instruction: String,
    pub distance_m: f64,
    pub waypoint_index: usize,
}

/// Snapped geometry for a route, computed by a routing engine or imported from GPX.
/// Stored alongside the route so that viewers don't need to re-snap on every render.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RouteGeometry {
    /// Encoded polyline (Google polyline algorithm, precision 6 for Valhalla output).
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub polyline: String,
    /// Source engine: "valhalla" | "manual" | "gpx" | other future values.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub engine: String,
    /// Engine-specific costing/profile, e.g. "pedestrian", "bicycle", "auto".
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub costing: Option<String>,
    /// Unix milliseconds when this geometry was computed.
    pub computed_at: i64,
}

/// User-created route (hiking, cycling, etc.)
/// URI: /pub/mapky.app/routes/:route_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppRoute {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub name: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub activity: RouteActivityType,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub waypoints: Vec<Waypoint>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub osm_ways: Option<Vec<String>>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub control_points: Option<Vec<Waypoint>>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub steps: Option<Vec<RouteStep>>,
    pub distance_m: Option<f64>,
    pub elevation_gain_m: Option<f64>,
    pub elevation_loss_m: Option<f64>,
    pub estimated_duration_s: Option<i64>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub image_uri: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub geometry: Option<RouteGeometry>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppRoute {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(name: String, activity: RouteActivityType, waypoints: Vec<Waypoint>) -> Self {
        let route = MapkyAppRoute {
            name,
            description: None,
            activity,
            waypoints,
            osm_ways: None,
            control_points: None,
            steps: None,
            distance_m: None,
            elevation_gain_m: None,
            elevation_loss_m: None,
            estimated_duration_s: None,
            image_uri: None,
            geometry: None,
        };
        route.sanitize()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppRoute {
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
impl Json for MapkyAppRoute {}

impl TimestampId for MapkyAppRoute {}

impl HasIdPath for MapkyAppRoute {
    const PATH_SEGMENT: &'static str = "routes/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppRoute {
    fn sanitize(self) -> Self {
        let name = self.name.trim().to_string();
        let description = self.description.map(|d| d.trim().to_string());
        let image_uri = self.image_uri.map(|u| sanitize_url(&u));
        let osm_ways = self
            .osm_ways
            .map(|ways| ways.into_iter().map(|u| sanitize_url(&u)).collect());

        MapkyAppRoute {
            name,
            description,
            image_uri,
            osm_ways,
            ..self
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        // Validate name
        if self.name.trim().is_empty() {
            return Err("Validation Error: Route name cannot be empty".into());
        }
        if self.name.chars().count() > MAX_ROUTE_NAME_LENGTH {
            return Err(format!(
                "Validation Error: Route name exceeds maximum length of {} characters",
                MAX_ROUTE_NAME_LENGTH
            ));
        }

        // Validate description
        if let Some(ref desc) = self.description {
            if desc.chars().count() > MAX_ROUTE_DESCRIPTION_LENGTH {
                return Err(format!(
                    "Validation Error: Description exceeds maximum length of {} characters",
                    MAX_ROUTE_DESCRIPTION_LENGTH
                ));
            }
        }

        // Validate waypoints
        if self.waypoints.len() < MIN_WAYPOINTS {
            return Err(format!(
                "Validation Error: Route must have at least {} waypoints",
                MIN_WAYPOINTS
            ));
        }
        if self.waypoints.len() > MAX_ROUTE_WAYPOINTS {
            return Err(format!(
                "Validation Error: Route exceeds maximum of {} waypoints",
                MAX_ROUTE_WAYPOINTS
            ));
        }

        for (i, wp) in self.waypoints.iter().enumerate() {
            validate_coordinates(wp.lat, wp.lon)
                .map_err(|e| format!("Validation Error: Waypoint {}: {}", i, e))?;
            if let Some(ref name) = wp.name {
                if name.chars().count() > MAX_WAYPOINT_NAME_LENGTH {
                    return Err(format!(
                        "Validation Error: Waypoint {} name exceeds maximum length of {} characters",
                        i, MAX_WAYPOINT_NAME_LENGTH
                    ));
                }
            }
        }

        // Validate control_points
        if let Some(ref cps) = self.control_points {
            if cps.len() > MAX_ROUTE_CONTROL_POINTS {
                return Err(format!(
                    "Validation Error: Route exceeds maximum of {} control points",
                    MAX_ROUTE_CONTROL_POINTS
                ));
            }
            if cps.len() < MIN_WAYPOINTS {
                return Err(format!(
                    "Validation Error: control_points must have at least {} points",
                    MIN_WAYPOINTS
                ));
            }
            for (i, cp) in cps.iter().enumerate() {
                validate_coordinates(cp.lat, cp.lon)
                    .map_err(|e| format!("Validation Error: control_points[{}]: {}", i, e))?;
            }
        }

        // Validate steps
        if let Some(ref steps) = self.steps {
            for (i, step) in steps.iter().enumerate() {
                if step.instruction.chars().count() > MAX_ROUTE_INSTRUCTION_LENGTH {
                    return Err(format!(
                        "Validation Error: steps[{}] instruction exceeds maximum length of {} characters",
                        i, MAX_ROUTE_INSTRUCTION_LENGTH
                    ));
                }
                if step.distance_m < 0.0 {
                    return Err(format!(
                        "Validation Error: steps[{}] distance_m cannot be negative",
                        i
                    ));
                }
                if step.waypoint_index >= self.waypoints.len() {
                    return Err(format!(
                        "Validation Error: steps[{}] waypoint_index {} out of bounds (route has {} waypoints)",
                        i, step.waypoint_index, self.waypoints.len()
                    ));
                }
            }
        }

        // Validate osm_ways — all must be Way URLs
        if let Some(ref ways) = self.osm_ways {
            for (i, way) in ways.iter().enumerate() {
                validate_osm_way_url(way)
                    .map_err(|e| format!("Validation Error: osm_ways[{}]: {}", i, e))?;
            }
        }

        // Validate image URI
        if let Some(ref uri) = self.image_uri {
            url::Url::parse(uri)
                .map_err(|_| format!("Validation Error: Invalid image URI: {}", uri))?;
        }

        // Validate non-negative measurements
        if let Some(d) = self.distance_m {
            if d < 0.0 {
                return Err("Validation Error: distance_m cannot be negative".into());
            }
        }
        if let Some(g) = self.elevation_gain_m {
            if g < 0.0 {
                return Err("Validation Error: elevation_gain_m cannot be negative".into());
            }
        }
        if let Some(l) = self.elevation_loss_m {
            if l < 0.0 {
                return Err("Validation Error: elevation_loss_m cannot be negative".into());
            }
        }
        if let Some(d) = self.estimated_duration_s {
            if d < 0 {
                return Err("Validation Error: estimated_duration_s cannot be negative".into());
            }
        }

        // Validate geometry
        if let Some(ref g) = self.geometry {
            if g.polyline.is_empty() {
                return Err("Validation Error: geometry.polyline cannot be empty".into());
            }
            if g.polyline.len() > MAX_ROUTE_POLYLINE_LENGTH {
                return Err(format!(
                    "Validation Error: geometry.polyline exceeds maximum length of {} bytes",
                    MAX_ROUTE_POLYLINE_LENGTH
                ));
            }
            if g.engine.trim().is_empty() {
                return Err("Validation Error: geometry.engine cannot be empty".into());
            }
            if g.engine.chars().count() > MAX_ROUTE_ENGINE_LENGTH {
                return Err(format!(
                    "Validation Error: geometry.engine exceeds maximum length of {} characters",
                    MAX_ROUTE_ENGINE_LENGTH
                ));
            }
            if let Some(ref costing) = g.costing {
                if costing.chars().count() > MAX_ROUTE_COSTING_LENGTH {
                    return Err(format!(
                        "Validation Error: geometry.costing exceeds maximum length of {} characters",
                        MAX_ROUTE_COSTING_LENGTH
                    ));
                }
            }
            if g.computed_at < 0 {
                return Err("Validation Error: geometry.computed_at cannot be negative".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_waypoints() -> Vec<Waypoint> {
        vec![
            Waypoint::new(47.3769, 8.5417, Some(400.0)),
            Waypoint::new(47.3800, 8.5450, Some(420.0)),
            Waypoint::new(47.3850, 8.5500, Some(450.0)),
        ]
    }

    #[test]
    fn test_create_id() {
        let route = MapkyAppRoute::new(
            "Lake Loop".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        let id = route.create_id();
        assert_eq!(id.len(), 13);
    }

    #[test]
    fn test_create_path() {
        let route = MapkyAppRoute::new(
            "Lake Loop".into(),
            RouteActivityType::Cycling,
            test_waypoints(),
        );
        let id = route.create_id();
        let path = MapkyAppRoute::create_path(&id);
        assert!(path.starts_with("/pub/mapky.app/routes/"));
    }

    #[test]
    fn test_validate_happy() {
        let route = MapkyAppRoute::new(
            "Lake Loop".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let route = MapkyAppRoute::new("".into(), RouteActivityType::Hiking, test_waypoints());
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_too_few_waypoints() {
        let route = MapkyAppRoute::new(
            "Short".into(),
            RouteActivityType::Walking,
            vec![Waypoint::new(0.0, 0.0, None)],
        );
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_invalid_waypoint_coords() {
        let route = MapkyAppRoute::new(
            "Bad Route".into(),
            RouteActivityType::Hiking,
            vec![
                Waypoint::new(0.0, 0.0, None),
                Waypoint::new(91.0, 0.0, None),
            ],
        );
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_osm_ways_must_be_way() {
        let mut route = MapkyAppRoute::new(
            "Linked Route".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        route.osm_ways = Some(vec!["https://www.openstreetmap.org/node/123".into()]);
        let id = route.create_id();
        let result = route.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Way URL"));
    }

    #[test]
    fn test_validate_osm_ways_valid() {
        let mut route = MapkyAppRoute::new(
            "Linked Route".into(),
            RouteActivityType::Cycling,
            test_waypoints(),
        );
        route.osm_ways = Some(vec![
            "https://www.openstreetmap.org/way/123".into(),
            "https://www.openstreetmap.org/way/456".into(),
        ]);
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_negative_distance() {
        let mut route =
            MapkyAppRoute::new("Route".into(), RouteActivityType::Hiking, test_waypoints());
        route.distance_m = Some(-1.0);
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_waypoint_with_name() {
        let mut wp = Waypoint::new(47.0, 8.0, None);
        wp.name = Some("Summit".into());
        let route = MapkyAppRoute::new(
            "Named WP Route".into(),
            RouteActivityType::Hiking,
            vec![wp, Waypoint::new(47.1, 8.1, None)],
        );
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_waypoint_name_too_long() {
        let mut wp = Waypoint::new(47.0, 8.0, None);
        wp.name = Some("a".repeat(101));
        let route = MapkyAppRoute::new(
            "Route".into(),
            RouteActivityType::Hiking,
            vec![wp, Waypoint::new(47.1, 8.1, None)],
        );
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_control_points() {
        let mut route =
            MapkyAppRoute::new("Route".into(), RouteActivityType::Hiking, test_waypoints());
        route.control_points = Some(vec![
            Waypoint::new(47.0, 8.0, None),
            Waypoint::new(47.1, 8.1, None),
        ]);
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_control_points_too_few() {
        let mut route =
            MapkyAppRoute::new("Route".into(), RouteActivityType::Hiking, test_waypoints());
        route.control_points = Some(vec![Waypoint::new(47.0, 8.0, None)]);
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_steps() {
        let mut route =
            MapkyAppRoute::new("Route".into(), RouteActivityType::Hiking, test_waypoints());
        route.steps = Some(vec![
            RouteStep {
                instruction: "Head north".into(),
                distance_m: 100.0,
                waypoint_index: 0,
            },
            RouteStep {
                instruction: "Turn right".into(),
                distance_m: 200.0,
                waypoint_index: 1,
            },
        ]);
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_step_waypoint_index_out_of_bounds() {
        let mut route =
            MapkyAppRoute::new("Route".into(), RouteActivityType::Hiking, test_waypoints());
        route.steps = Some(vec![RouteStep {
            instruction: "Go".into(),
            distance_m: 10.0,
            waypoint_index: 99,
        }]);
        let id = route.create_id();
        let result = route.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of bounds"));
    }

    #[test]
    fn test_validate_step_negative_distance() {
        let mut route =
            MapkyAppRoute::new("Route".into(), RouteActivityType::Hiking, test_waypoints());
        route.steps = Some(vec![RouteStep {
            instruction: "Go".into(),
            distance_m: -5.0,
            waypoint_index: 0,
        }]);
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_all_activity_types() {
        let types = vec![
            RouteActivityType::Hiking,
            RouteActivityType::Cycling,
            RouteActivityType::Running,
            RouteActivityType::Walking,
            RouteActivityType::Driving,
            RouteActivityType::Skiing,
            RouteActivityType::Other,
        ];
        for activity in types {
            let route = MapkyAppRoute::new("Test".into(), activity, test_waypoints());
            let id = route.create_id();
            assert!(route.validate(Some(&id)).is_ok());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut route = MapkyAppRoute::new(
            "Mountain Trail".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        route.distance_m = Some(12500.0);

        let json = serde_json::to_string(&route).unwrap();
        assert!(json.contains("\"hiking\""));
        assert!(json.contains("\"distance_m\":12500"));
        let parsed: MapkyAppRoute = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "Mountain Trail");
        assert_eq!(parsed.distance_m, Some(12500.0));
    }

    /// Existing routes saved with `difficulty` should still deserialize
    /// cleanly — serde drops unknown fields by default. We don't read the
    /// value anywhere; users should re-tag with universal tags instead.
    #[test]
    fn test_legacy_difficulty_field_ignored() {
        let json = r#"{
            "name": "Legacy",
            "activity": "hiking",
            "difficulty": "expert",
            "waypoints": [
                {"lat": 47.3, "lon": 8.5},
                {"lat": 47.4, "lon": 8.6}
            ]
        }"#;
        let parsed: MapkyAppRoute = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.name, "Legacy");
    }

    #[test]
    fn test_geometry_roundtrip() {
        let mut route = MapkyAppRoute::new(
            "With Geometry".into(),
            RouteActivityType::Cycling,
            test_waypoints(),
        );
        route.geometry = Some(RouteGeometry {
            polyline: "kpkfFcueeBgC@".into(),
            engine: "valhalla".into(),
            costing: Some("bicycle".into()),
            computed_at: 1_730_000_000_000,
        });
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_ok());

        let json = serde_json::to_string(&route).unwrap();
        assert!(json.contains("\"valhalla\""));
        assert!(json.contains("\"computed_at\":1730000000000"));
        let parsed: MapkyAppRoute = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.geometry.as_ref().unwrap().engine, "valhalla");
        assert_eq!(
            parsed.geometry.as_ref().unwrap().costing.as_deref(),
            Some("bicycle")
        );
    }

    #[test]
    fn test_geometry_validation_empty_polyline() {
        let mut route = MapkyAppRoute::new(
            "Bad Geo".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        route.geometry = Some(RouteGeometry {
            polyline: String::new(),
            engine: "valhalla".into(),
            costing: None,
            computed_at: 0,
        });
        let id = route.create_id();
        assert!(route.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_geometry_validation_oversize_polyline() {
        let mut route = MapkyAppRoute::new(
            "Big Geo".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        route.geometry = Some(RouteGeometry {
            polyline: "a".repeat(MAX_ROUTE_POLYLINE_LENGTH + 1),
            engine: "valhalla".into(),
            costing: None,
            computed_at: 0,
        });
        let id = route.create_id();
        let err = route.validate(Some(&id)).unwrap_err();
        assert!(err.contains("polyline"));
    }

    #[test]
    fn test_geometry_validation_empty_engine() {
        let mut route = MapkyAppRoute::new(
            "No Engine".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        route.geometry = Some(RouteGeometry {
            polyline: "kpkfFcueeB".into(),
            engine: "   ".into(),
            costing: None,
            computed_at: 0,
        });
        let id = route.create_id();
        let err = route.validate(Some(&id)).unwrap_err();
        assert!(err.contains("engine"));
    }

    #[test]
    fn test_geometry_validation_negative_computed_at() {
        let mut route = MapkyAppRoute::new(
            "Negative Time".into(),
            RouteActivityType::Hiking,
            test_waypoints(),
        );
        route.geometry = Some(RouteGeometry {
            polyline: "kpkfFcueeB".into(),
            engine: "valhalla".into(),
            costing: None,
            computed_at: -1,
        });
        let id = route.create_id();
        let err = route.validate(Some(&id)).unwrap_err();
        assert!(err.contains("computed_at"));
    }

    #[test]
    fn test_try_from_valid() {
        let json = r#"{
            "name": "Lake Loop",
            "description": "A nice walk around the lake",
            "activity": "hiking",
            "waypoints": [
                {"lat": 47.3769, "lon": 8.5417, "ele": 400.0},
                {"lat": 47.3800, "lon": 8.5450, "ele": 420.0}
            ],
            "osm_ways": null,
            "distance_m": 5000.0,
            "elevation_gain_m": 100.0,
            "elevation_loss_m": 100.0,
            "estimated_duration_s": 3600,
            "image_uri": null
        }"#;
        let route = MapkyAppRoute::new(
            "Lake Loop".into(),
            RouteActivityType::Hiking,
            vec![
                Waypoint::new(47.3769, 8.5417, Some(400.0)),
                Waypoint::new(47.3800, 8.5450, Some(420.0)),
            ],
        );
        let id = route.create_id();
        let result = <MapkyAppRoute as Validatable>::try_from(json.as_bytes(), &id);
        assert!(result.is_ok());
    }
}
