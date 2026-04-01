> **Warning:** This project is in early development. APIs and data models may change without notice.

# mapky-app-specs

Rust/WASM type library defining all MapKy data models. Extends [pubky-app-specs](https://github.com/pubky/pubky-app-specs) traits (`Validatable`, `TimestampId`, `HashId`, `HasIdPath`).

MapKy is a decentralized social layer on OpenStreetMap using [Pubky](https://pubky.tech).

## Models

| Model | Path | Description |
|---|---|---|
| `MapkyAppPost` | `/pub/mapky.app/posts/<id>` | Reviews, questions, comments about places (anchored to OSM URL) |
| `MapkyAppCollection` | `/pub/mapky.app/collections/<id>` | Named lists of places (OSM URLs) |
| `MapkyAppIncident` | `/pub/mapky.app/incidents/<id>` | Waze-style crowdsourced hazard reports |
| `MapkyAppGeoCapture` | `/pub/mapky.app/geo_captures/<id>` | Street-level media (photos, panoramas, 3D) |
| `MapkyAppRoute` | `/pub/mapky.app/routes/<id>` | User-created hiking/cycling/driving routes |

Places are identified by their canonical OpenStreetMap URL (e.g. `https://www.openstreetmap.org/node/123`). Tags on places use standard `PubkyAppTag` (universal tags) stored at `/pub/mapky.app/tags/`.

## Build

```sh
# Run tests
cargo test

# Build WASM package
wasm-pack build --target bundler
```
