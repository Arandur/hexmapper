use iced::{Sandbox, Settings};

use hexmapper::application::AppState;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(AppState::run(Settings::default())?)
}
