use iced::{Border, Color};
use widgets::table_row;

use super::Theme;

impl table_row::style::Stylesheet for Theme {
    type Style = ();

    fn active(&self) -> table_row::style::Appearance {
        table_row::style::Appearance {
            background: iced::Background::Color(self.palette.background_light),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 5.0.into(),
            },
        }
    }
}
