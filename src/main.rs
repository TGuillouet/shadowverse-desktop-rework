use data::{config::Config, db::setup_db};
use iced::{window, Application, Settings, Size};

mod app;
mod executor;
mod screens;
mod theme;
mod widget;
mod widgets;

fn main() -> Result<(), iced::Error> {
    let config = Config::load().unwrap();

    let _ = setup_db(&config);

    let default_settings = Settings::<()>::default();
    let window = window::Settings {
        size: Size {
            width: 1280.0,
            height: 900.0,
        },
        ..default_settings.clone().window
    };
    let settings = Settings {
        id: default_settings.id,
        window,
        flags: config,
        fonts: default_settings.fonts,
        default_font: default_settings.default_font,
        default_text_size: default_settings.default_text_size,
        antialiasing: default_settings.antialiasing,
    };

    app::IcedApplication::run(settings)
}
