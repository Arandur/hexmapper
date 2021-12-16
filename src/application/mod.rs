mod canvas;

use iced::canvas::Canvas;
use iced::{Container, Element, Length, Sandbox};

use serde::{Deserialize, Serialize};

use canvas::HexCanvas;

use crate::hex::Hex;

use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
  version: Cow<'static, str>,
  hexes: Vec<Hex>,
}

impl State {
  pub fn load<P: AsRef<Path>>(path: P) -> Result<State, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let state = serde_json::from_reader(reader)?;

    Ok(state)
  }
}

impl Sandbox for State {
  type Message = ();

  fn new() -> State {
    State::load("resources/sample_map.json").expect("Could not load state")
  }

  fn title(&self) -> String {
    String::from("A cool application")
  }

  fn update(&mut self, _message: Self::Message) {
    // No interactions
  }

  fn view(&mut self) -> Element<Self::Message> {
    let canvas = Canvas::new(HexCanvas {
      hexes: &self.hexes,
      selected: None,
    })
    .width(Length::Fill)
    .height(Length::Fill);

    Container::new(canvas)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }
}
