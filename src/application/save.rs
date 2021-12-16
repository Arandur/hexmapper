use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use super::AppState;
use crate::hex::Hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
  version: String,
  hexes: Vec<Hex>,
}

impl SaveState {
  pub fn load<P: AsRef<Path>>(path: P) -> Result<SaveState, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let state = serde_json::from_reader(reader)?;

    Ok(state)
  }

  pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, self)?;

    Ok(())
  }

  pub fn into_application_state(self) -> AppState {
    AppState {
      hexes: self
        .hexes
        .into_iter()
        .map(|hex| (hex.coordinate, hex))
        .collect(),
    }
  }
}
