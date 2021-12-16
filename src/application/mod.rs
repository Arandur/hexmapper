mod canvas;
mod save;

use iced::canvas::Canvas;
use iced::{Container, Element, Length, Sandbox};

use canvas::HexCanvas;

use crate::hex::{Hex, HexCoordinate};
use save::SaveState;

use std::collections::HashMap;

#[derive(Debug)]
pub struct AppState {
  pub(crate) hexes: HashMap<HexCoordinate, Hex>,
}

impl Sandbox for AppState {
  type Message = ();

  fn new() -> AppState {
    SaveState::load("resources/sample_map.json")
      .expect("Could not load state")
      .into_application_state()
  }

  fn title(&self) -> String {
    String::from("Hexmapper")
  }

  fn update(&mut self, _message: Self::Message) {
    // No interactions
  }

  fn view(&mut self) -> Element<Self::Message> {
    let canvas = Canvas::new(HexCanvas::new(&self.hexes))
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
