use data::{config::Config, db::setup_db};
use iced::{Application, Settings};

mod app;
mod screens;
mod theme;
mod widget;
mod widgets;

fn main() -> Result<(), iced::Error> {
    let config = Config::load().unwrap();

    setup_db(&config);

    let cards_list = cards_updater::get_cards(); // TODO: Fetch all the cards only on the first app init

    let default_settings = Settings::<()>::default();
    let settings = Settings {
        id: default_settings.id,
        window: default_settings.window,
        flags: config,
        fonts: default_settings.fonts,
        default_font: default_settings.default_font,
        default_text_size: default_settings.default_text_size,
        antialiasing: default_settings.antialiasing,
    };

    app::IcedApplication::run(settings)
}
