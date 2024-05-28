use iced::{widget::button, Border, Vector};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Extension,
    Sidebar,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: None,
                text_color: self.palette.light_primary,
                ..Default::default()
            },
            Button::Extension => button::Appearance {
                background: Some(self.palette.background.into()),
                border: Border {
                    color: self.palette.light_primary,
                    width: 1.0,
                    radius: 10.0.into(),
                },
                text_color: self.palette.light_primary,
                shadow_offset: Vector::new(5.0, 5.0),
                ..Default::default()
            },
            Button::Sidebar => button::Appearance {
                text_color: self.palette.primary,
                border: Border {
                    color: self.palette.primary,
                    radius: 0.0.into(),
                    width: 1.0,
                },
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active_style = self.active(style);
        match style {
            Button::Extension => button::Appearance {
                border: Border {
                    width: 3.0,
                    ..active_style.border
                },
                ..active_style
            },
            Button::Primary => button::Appearance {
                background: Some(iced::Background::Color(self.palette.light_primary)),
                text_color: self.palette.dark_primary,
                border: Border::default(),
                ..Default::default()
            },
            _ => self.active(style),
        }
    }
}
