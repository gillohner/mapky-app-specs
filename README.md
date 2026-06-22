> **Warning:** This project is in early development. APIs and data models may change without notice.

# mapky-app-specs

Rust/WASM type library defining all MapKy data models. Extends [pubky-app-specs](https://github.com/pubky/pubky-app-specs) traits (`Validatable`, `TimestampId`, `HashId`, `HasIdPath`).

MapKy is a decentralized social layer on OpenStreetMap using [Pubky](https://pubky.tech).

## Models

| Model | Path | Anchored to | Description |
|---|---|---|---|
| `MapkyAppReview` | `/pub/mapky.app/reviews/<id>` | OSM URL | Reviews, questions, comments about places |
| `MapkyAppIncident` | `/pub/mapky.app/incidents/<id>` | lat/lon | Waze-style crowdsourced hazard report |
| `MapkyAppGeoCapture` | `/pub/mapky.app/geo_captures/<id>` | lat/lon | Street-level media (photos, panoramas, video, 3D, point cloud, audio) |
| `MapkyAppSequence` | `/pub/mapky.app/sequences/<id>` | ordered geo-capture list | Continuous trajectory of captures (e.g. driving panorama runs) |
| `MapkyAppRoute` | `/pub/mapky.app/routes/<id>` | `Vec<Waypoint>` + encoded polyline | User-created hiking / cycling / driving / running / walking route |

Places are identified by their canonical OpenStreetMap URL (e.g. `https://www.openstreetmap.org/node/123`). Tags on places, posts, routes, etc. use standard `PubkyAppTag` (universal tags) stored at `/pub/mapky.app/tags/` — the [mapky-nexus-plugin](https://github.com/gillohner/mapky-nexus-plugin) resolves these cross-domain so any MapKy resource is taggable.

## Routes

`MapkyAppRoute` carries the full trip: name, description, activity (`Walking | Running | Hiking | Cycling | Driving | Skiing | Other`), waypoints (with optional OSM anchors), an encoded polyline of the snapped path, distance / duration / elevation aggregates, and an optional cover image URI. The blob lives on the author's homeserver; the indexer mirrors a metadata + bbox subset into Neo4j for spatial discovery, and the frontend fetches the full body directly when rendering a route detail.

## Build

```sh
# Run tests
cargo test

# Build WASM package
wasm-pack build --target bundler
```
