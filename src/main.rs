use serde::{Deserialize, Serialize};

use iced::canvas::{event, Canvas, Cursor, Event, Frame, Geometry, Path, Program, Stroke};
use iced::{mouse, Color, Container, Element, Length, Point, Rectangle, Sandbox, Settings, Vector};

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct State {
    version: Cow<'static, str>,
    hexes: Vec<Hex>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Hex {
    coordinate: HexCoordinate,
    terrain: String,
    difficulty: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct HexCoordinate {
    q: i64,
    r: i64,
    s: i64,
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

impl State {
    fn load<P: AsRef<path::Path>>(path: P) -> Result<State, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let state = serde_json::from_reader(reader)?;

        Ok(state)
    }
}

struct HexCanvas<'a> {
    hexes: &'a [Hex],
    selected: Option<HexCoordinate>,
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
                        y: hex_radius
                            * (f32::sqrt(3.0) / 2.0 * q as f32 + f32::sqrt(3.0) * r as f32),
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
                        y: hex_radius
                            * (f32::sqrt(3.0) / 2.0 * q as f32 + f32::sqrt(3.0) * r as f32),
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

fn main() -> Result<(), Box<dyn Error>> {
    Ok(State::run(Settings::default())?)
}
