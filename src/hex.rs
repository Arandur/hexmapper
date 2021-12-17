use serde::{Deserialize, Serialize};
use serde_json::Value;

use iced::{Point, Vector};

use euclid::{Point2D, Vector2D};

use std::collections::HashMap;

pub struct HexSpace;
pub type HexPoint = Point2D<f32, HexSpace>;
pub type HexVector = Vector2D<f32, HexSpace>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hex {
  pub coordinate: HexCoordinate,

  #[serde(flatten)]
  pub data: HashMap<String, Value>,
}

impl Hex {
  pub fn new(coordinate: HexCoordinate) -> Hex {
    Hex {
      coordinate,
      data: HashMap::new(),
    }
  }

  pub fn center(&self) -> HexPoint {
    HexPoint::new(
      1.5 * self.coordinate.q as f32,
      f32::sqrt(3.0) * (0.5 * self.coordinate.q as f32 + self.coordinate.r as f32),
    )
  }

  pub fn vertices() -> impl Iterator<Item = HexVector> {
    (0..6).into_iter().map(move |i| {
      let (sin, cos) = f32::sin_cos(i as f32 * std::f32::consts::FRAC_PI_3);

      HexVector::new(cos, sin)
    })
  }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<HexPoint> for HexCoordinate {
  fn from(point: HexPoint) -> HexCoordinate {
    let q = -2.0 * point.x / 3.0;
    let r = (point.x + f32::sqrt(3.0) * point.y) / 3.0;
    let s = 0.0 - q - r;

    let q_i = f32::round(q);
    let r_i = f32::round(r);
    let s_i = f32::round(s);

    let q_d = f32::abs(q_i - q);
    let r_d = f32::abs(r_i - r);
    let s_d = f32::abs(s_i - s);

    let mut q = q_i as i64;
    let mut r = r_i as i64;
    let mut s = s_i as i64;

    if q_d > r_d && q_d > s_d {
      q = 0 - r - s;
    } else if r_d > s_d {
      r = 0 - q - s;
    } else {
      s = 0 - q - r;
    }

    HexCoordinate { q, r, s }
  }
}
