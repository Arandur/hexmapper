use iced::canvas::{self, event, Cursor, Event, Geometry, Path, Program, Stroke};
use iced::{mouse, Color, Rectangle, Vector};

use crate::hex::{Hex, HexCoordinate};

use std::collections::HashMap;

struct Cache {
  hex_outlines: canvas::Cache,
  selected_hex_outline: canvas::Cache,
}

impl Cache {
  fn new() -> Cache {
    Cache {
      hex_outlines: canvas::Cache::new(),
      selected_hex_outline: canvas::Cache::new(),
    }
  }
}

pub struct HexCanvas<'a> {
  hexes: &'a HashMap<HexCoordinate, Hex>,
  selected: Option<HexCoordinate>,
  cache: Cache,
}

impl<'a> HexCanvas<'a> {
  pub fn new(hexes: &'a HashMap<HexCoordinate, Hex>) -> HexCanvas {
    HexCanvas {
      hexes,
      selected: None,
      cache: Cache::new(),
    }
  }
}

impl<'a> Program<()> for HexCanvas<'a> {
  fn update(
    &mut self,
    event: Event,
    bounds: Rectangle,
    cursor: Cursor,
  ) -> (event::Status, Option<()>) {
    let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
      position
    } else {
      return (event::Status::Ignored, None);
    };

    let center = bounds.center();
    let radius = bounds.width.min(bounds.height) / 2.0;
    let hex_radius = radius / 10.0;
    let Vector { x, y } = center - cursor_position;

    match event {
      Event::Mouse(mouse_event) => {
        if let mouse::Event::ButtonPressed(mouse::Button::Left) = mouse_event {
          // if the cursor is positioned over a hex, return a Message::SelectedHex
          let q = 2.0 * x / (3.0 * hex_radius);
          let r = -(x + f32::sqrt(3.0) * y) / (3.0 * hex_radius);
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

          let coord = HexCoordinate::new(q, r, s);

          if self.hexes.contains_key(&coord) {
            self.selected = Some(coord);
            self.cache.selected_hex_outline.clear();
          } else {
            self.selected = None;
          }
        }

        (event::Status::Captured, None)
      }
      _ => (event::Status::Ignored, None),
    }
  }

  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    let center = bounds.center();
    let radius = f32::min(bounds.width, bounds.height) / 2.0;
    let hex_radius = radius / 10.0;

    let hex_outlines = self.cache.hex_outlines.draw(bounds.size(), |frame| {
      let hexes = Path::new(|builder| {
        for coordinate in self.hexes.keys() {
          let hex_center = center + coordinate.as_vector() * hex_radius;

          let mut vertices = (0..6).into_iter().map(|i| {
            let (sin, cos) = f32::sin_cos(i as f32 * std::f32::consts::FRAC_PI_3);

            hex_center
              + Vector {
                x: hex_radius * cos,
                y: hex_radius * sin,
              }
          });

          builder.move_to(vertices.next().unwrap());

          for vertex in vertices {
            builder.line_to(vertex);
          }

          builder.close();
        }
      });

      let hex_stroke = Stroke {
        color: Color::BLACK,
        ..Default::default()
      };

      frame.stroke(&hexes, hex_stroke);
    });

    let selected_hex_outline = self
      .cache
      .selected_hex_outline
      .draw(bounds.size(), |frame| {
        if let Some(coord) = self.selected {
          let selected_hex_stroke = Stroke {
            color: Color::new(1.0, 0.0, 0.0, 1.0),
            ..Default::default()
          };

          let path = Path::new(|builder| {
            let hex_center = center + coord.as_vector() * hex_radius;

            let mut vertices = (0..6).into_iter().map(|i| {
              let (sin, cos) = f32::sin_cos(i as f32 * std::f32::consts::FRAC_PI_3);

              hex_center
                + Vector {
                  x: hex_radius * cos,
                  y: hex_radius * sin,
                }
            });

            builder.move_to(vertices.next().unwrap());

            for vertex in vertices {
              builder.line_to(vertex);
            }

            builder.close();
          });

          frame.stroke(&path, selected_hex_stroke);
        }
      });

    vec![hex_outlines, selected_hex_outline]
  }
}
