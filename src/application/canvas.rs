use iced::canvas::{self, event, Cursor, Event, Geometry, Path, Program, Stroke};
use iced::{mouse, Color, Point, Rectangle, Vector};

use euclid::{Point2D, Transform2D, Vector2D};

use crate::hex::{Hex, HexCoordinate, HexSpace};

use std::collections::HashMap;

struct HexCanvasSpace;
type HexCanvasPoint = Point2D<f32, HexCanvasSpace>;
type HexCanvasVector = Vector2D<f32, HexCanvasSpace>;

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
  transform: Transform2D<f32, HexSpace, HexCanvasSpace>,
  cache: Cache,
}

impl<'a> HexCanvas<'a> {
  pub fn new(hexes: &'a HashMap<HexCoordinate, Hex>) -> HexCanvas {
    HexCanvas {
      hexes,
      selected: None,
      transform: Transform2D::identity(),
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
      HexCanvasPoint::new(position.x, position.y)
    } else {
      return (event::Status::Ignored, None);
    };

    let center = bounds.center();
    let center = HexCanvasPoint::new(center.x, center.y);
    let radius = bounds.width.min(bounds.height) / 2.0;
    let hex_radius = radius / 10.0;

    self.transform = Transform2D::scale(hex_radius, hex_radius)
      .then_translate(HexCanvasVector::new(center.x, center.y));

    match event {
      Event::Mouse(mouse_event) => {
        if let mouse::Event::ButtonPressed(mouse::Button::Left) = mouse_event {
          let coord: HexCoordinate = self
            .transform
            .inverse()
            .unwrap()
            .transform_point(cursor_position)
            .into();

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

    let transform: Transform2D<f32, HexSpace, HexCanvasSpace> =
      Transform2D::scale(hex_radius, hex_radius)
        .then_translate(HexCanvasVector::new(center.x, center.y));

    let hex_outlines = self.cache.hex_outlines.draw(bounds.size(), |frame| {
      let hexes = Path::new(|builder| {
        for hex in self.hexes.values() {
          let hex_center = transform.transform_point(hex.center());

          let mut vertices = Hex::vertices()
            .map(|v| transform.transform_vector(v))
            .map(|v| hex_center + v);

          let vertex = vertices.next().unwrap();
          builder.move_to(Point::new(vertex.x, vertex.y));

          for vertex in vertices {
            builder.line_to(Point::new(vertex.x, vertex.y));
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

          let hex = self.hexes.get(&coord).unwrap();

          let path = Path::new(|builder| {
            let hex_center = transform.transform_point(hex.center());

            let mut vertices = Hex::vertices()
              .map(|v| transform.transform_vector(v))
              .map(|v| hex_center + v);

            let vertex = vertices.next().unwrap();
            builder.move_to(Point::new(vertex.x, vertex.y));

            for vertex in vertices {
              builder.line_to(Point::new(vertex.x, vertex.y));
            }

            builder.close();
          });

          frame.stroke(&path, selected_hex_stroke);
        }
      });

    vec![hex_outlines, selected_hex_outline]
  }
}
