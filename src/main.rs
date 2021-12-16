use iced::{Sandbox, Settings};

use hexmapper::application::State;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(State::run(Settings::default())?)
}
