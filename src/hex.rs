use serde::{Deserialize, Serialize};
use serde_json::Value;

use iced::{Point, Vector};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hex {
  pub coordinate: HexCoordinate,

  #[serde(flatten)]
  pub data: HashMap<String, Value>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexCoordinate {
  q: i64,
  r: i64,
  s: i64,
}

impl HexCoordinate {
  pub fn new(q: i64, r: i64, s: i64) -> HexCoordinate {
    if q + r + s != 0 {
      panic!("Invalid hex coordinate");
    }

    HexCoordinate { q, r, s }
  }

  pub fn as_point(&self) -> Point {
    Point::new(
      1.5 * self.q as f32,
      f32::sqrt(3.0) * (0.5 * self.q as f32 + self.r as f32),
    )
  }

  pub fn as_vector(&self) -> Vector {
    Vector::new(
      1.5 * self.q as f32,
      f32::sqrt(3.0) * (0.5 * self.q as f32 + self.r as f32),
    )
  }
}
