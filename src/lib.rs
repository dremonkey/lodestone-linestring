/// The main crate for lodestone-linestring
///
/// ## Overview
/// 
/// Takes coordinates and returns a new LineString GeoJson feature.
/// Inspired by [turf-linestring](https://github.com/Turfjs/turf-linestring).

// Third party packages
extern crate geojson;
extern crate rustc_serialize;

use rustc_serialize::json;
use geojson::{Feature, GeoJson, Geometry, Value};

pub extern fn linestring(
  coordinates: Vec<Vec<f64>>) -> GeoJson {

  assert!(coordinates.len() >= 2);

  let geometry = Geometry::new(Value::LineString(coordinates));
  let properties = json::Object::new();

  GeoJson::Feature(Feature {
    bbox: None,
    crs: None,
    geometry: geometry,
    id: None,
    properties: Some(properties),
  })
}

#[cfg(Test)]
mod test {
  use rustc_serialize::json::{self, ToJson};
  use super::linestring;

  #[test]
  fn test_valid_coordinates() {
    
    let expected_json = "{\"geometry\":{\"coordinates\":[[-1,0,1.0],[-2.0,2.0]],\"type\":\"LineString\"},\"properties\":{},\"type\":\"Feature\"}";

    let coords = vec![vec![-1.0, 1.0], vec![-2.0, 2.0]];
    let geojson = linestring(coords);
    let linestring_str = json::encode(&geojson.to_json()).unwrap();

    assert_eq!(linestring_str, expected_json);
  }

  #[test]
  #[should_panic]
  fn test_invalid_coordinates() {
    let coords = vec![vec![1.0, 1.0]];
    linestring(coords);
  }
}