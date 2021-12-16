use iced::canvas::{event, Cursor, Event, Frame, Geometry, Path, Program, Stroke};
use iced::{mouse, Color, Rectangle, Vector};

use crate::hex::{Hex, HexCoordinate};

pub struct HexCanvas<'a> {
  pub hexes: &'a [Hex],
  pub selected: Option<HexCoordinate>,
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

          let coord = if q_d > r_d && q_d > s_d {
            HexCoordinate {
              q: (-r_i - s_i) as i64,
              r: r_i as i64,
              s: s_i as i64,
            }
          } else if r_d > s_d {
            HexCoordinate {
              q: q_i as i64,
              r: (-q_i - s_i) as i64,
              s: s_i as i64,
            }
          } else {
            HexCoordinate {
              q: q_i as i64,
              r: r_i as i64,
              s: (-q_i - r_i) as i64,
            }
          };

          if self.hexes.iter().any(|h| h.coordinate == coord) {
            self.selected = Some(coord);
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
    let mut frame = Frame::new(bounds.size());

    let center = frame.center();
    let radius = frame.width().min(frame.height()) / 2.0;

    let hex_radius = radius / 10.0;

    let hexes = Path::new(|builder| {
      for hex in self.hexes {
        let HexCoordinate { q, r, .. } = hex.coordinate;
        let hex_center = center
          + Vector {
            x: hex_radius * 1.5 * q as f32,
            y: hex_radius * (f32::sqrt(3.0) / 2.0 * q as f32 + f32::sqrt(3.0) * r as f32),
          };

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

    if let Some(coord) = self.selected {
      let selected_hex_stroke = Stroke {
        color: Color::new(1.0, 0.0, 0.0, 1.0),
        ..Default::default()
      };

      let path = Path::new(|builder| {
        let HexCoordinate { q, r, .. } = coord;
        let hex_center = center
          + Vector {
            x: hex_radius * 1.5 * q as f32,
            y: hex_radius * (f32::sqrt(3.0) / 2.0 * q as f32 + f32::sqrt(3.0) * r as f32),
          };

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

    vec![frame.into_geometry()]
  }
}
