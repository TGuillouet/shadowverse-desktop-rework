use iced::{color, widget::text_input, Border, Color};

use super::Theme;

#[derive(Debug, Default, Clone, Copy)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::WHITE.into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: Color::WHITE,
            },
            icon_color: Color::WHITE,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::BLACK.into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: self.palette.primary,
            },
            icon_color: Color::WHITE,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette.secondary_text
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.palette.dark_primary
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.palette.accent
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: color!(0x28, 0x28, 0x28).into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: color!(0x45, 0x85, 0x88),
            },
            icon_color: Color::WHITE,
        }
    }
}
