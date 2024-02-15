use iced::{Application, Settings};

mod app;
mod theme;

fn main() -> Result<(), iced::Error> {
    app::IcedApplication::run(Settings::default())
}
