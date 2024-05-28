use iced::application;

use super::Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette.background,
            text_color: self.palette.primary_text,
        }
    }
}
