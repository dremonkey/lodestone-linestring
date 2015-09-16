/// The main crate for lodestone-linestring
///
/// ## Overview
/// 
/// Takes coordinates and returns a new LineString GeoJson feature.
/// Inspired by [turf-linestring](https://github.com/Turfjs/turf-linestring).

// Standard lib crates
use std::str::FromStr;

// Third party crates
extern crate geojson;
extern crate rustc_serialize;

use rustc_serialize::json::{self, ToJson};
use geojson::{Error, Feature, Geometry, LineStringType, Value, FromObject};

pub struct FeatureLineString {
  feature: Feature
}

impl FeatureLineString {
  pub fn new(coordinates: LineStringType) -> Self {

    assert!(coordinates.len() >= 2, "LineString must have two or more coordinates");

    let geometry = Geometry::new(Value::LineString(coordinates));
    let properties = json::Object::new();

    FeatureLineString {
      feature: Feature {
        bbox: None,
        crs: None,
        geometry: geometry,
        id: None,
        properties: Some(properties),
      }
    }
  }

  pub fn coordinates(&self) -> &LineStringType {
    type Err = Error;
    
    match self.feature.geometry.value {
      Value::LineString(ref coords) => coords,
      _ => unreachable!("Type other than Value::LineString should not be possible"),
    }
  }
}

impl FromStr for FeatureLineString {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {

    let decoded_json = match json::Json::from_str(s) {
      Ok(j) => j,
      Err(..) => return Err(Error::new("Encountered malformed JSON")),
    };
    
    let object = match decoded_json {
      json::Json::Object(object) => object,
      _ => return Err(Error::new("Attempted to create GeoJSON from JSON that is not an object")),
    };

    Self::from_object(&object)
  }
}

impl FromObject for FeatureLineString {
  fn from_object(object: &json::Object) -> Result<Self, Error> {
    let feature = Feature::from_object(object).unwrap();
    Ok(FeatureLineString {
      feature: feature
    })
  }
}

impl ToJson for FeatureLineString {
  fn to_json(&self) -> json::Json {
    self.feature.to_json()
  }
}

impl ToString for FeatureLineString {
  fn to_string(&self) -> String {
    self.to_json().to_string()
  }
}

#[cfg(test)]
mod tests {
  use rustc_serialize::json::{self, ToJson};
  use super::FeatureLineString;

  #[test]
  fn test_valid_coordinates() {
    
    let expected_json = "{\"geometry\":{\"coordinates\":[[-1.0,1.0],[-2.0,2.0]],\"type\":\"LineString\"},\"properties\":{},\"type\":\"Feature\"}";

    let coords = vec![vec![-1.0, 1.0], vec![-2.0, 2.0]];
    let geojson = FeatureLineString::new(coords);
    let linestring_str = json::encode(&geojson.to_json()).unwrap();

    assert_eq!(linestring_str, expected_json);
  }

  #[test]
  #[should_panic(expected = "LineString must have two or more coordinates")]
  fn test_invalid_coordinates() {
    let coords = vec![vec![1.0, 1.0]];
    FeatureLineString::new(coords);
  }
}