mod canvas;
mod save;

use iced::canvas::Canvas;
use iced::{Container, Element, Length, Sandbox};

use canvas::HexCanvas;

use crate::hex::Hex;
use save::SaveState;

#[derive(Debug)]
pub struct AppState {
  pub(crate) hexes: Vec<Hex>,
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
