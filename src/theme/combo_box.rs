use iced::{overlay, Border, Color};

use super::Theme;

impl overlay::menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> overlay::menu::Appearance {
        overlay::menu::Appearance {
            text_color: self.palette.primary_text,
            background: iced::Background::Color(self.palette.background),
            border: Border {
                color: Color::WHITE,
                width: 0.0,
                radius: 0.0.into(),
            },
            selected_text_color: self.palette.light_primary,
            selected_background: iced::Background::Color(self.palette.background_light),
        }
    }
}
