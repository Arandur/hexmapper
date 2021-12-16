use serde::{Deserialize, Serialize};

use iced::{Point, Vector};

use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hex {
  pub coordinate: HexCoordinate,
  terrain: String,
  difficulty: String,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum HexTerrain {
  Aquatic,
  Arctic,
  Desert,
  Forest,
  Moutain,
  Plain,
  Swamp,
}

impl HexTerrain {
  const ALL: [HexTerrain; 7] = [
    HexTerrain::Aquatic,
    HexTerrain::Arctic,
    HexTerrain::Desert,
    HexTerrain::Forest,
    HexTerrain::Moutain,
    HexTerrain::Plain,
    HexTerrain::Swamp,
  ];
}

impl fmt::Display for HexTerrain {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        HexTerrain::Aquatic => "Aquatic",
        HexTerrain::Arctic => "Arctic",
        HexTerrain::Desert => "Desert",
        HexTerrain::Forest => "Forest",
        HexTerrain::Moutain => "Mountain",
        HexTerrain::Plain => "Plain",
        HexTerrain::Swamp => "Swamp",
      }
    )
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum HexTerrainDifficulty {
  Open,
  Difficult,
  GreaterDifficult,
}

impl HexTerrainDifficulty {
  const ALL: [HexTerrainDifficulty; 3] = [
    HexTerrainDifficulty::Open,
    HexTerrainDifficulty::Difficult,
    HexTerrainDifficulty::GreaterDifficult,
  ];
}

impl fmt::Display for HexTerrainDifficulty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        HexTerrainDifficulty::Open => "Open",
        HexTerrainDifficulty::Difficult => "Difficult",
        HexTerrainDifficulty::GreaterDifficult => "Greater Difficult",
      }
    )
  }
}
