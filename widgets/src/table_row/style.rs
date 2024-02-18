use iced_core::{color, Background, Border, Color};

pub struct Appearance {
    pub background: Background,
    pub border: Border,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: Background::Color(Color::WHITE),
            border: Border {
                color: Color::BLACK,
                radius: 5.0.into(),
                width: 1.0,
            },
        }
    }
}

pub trait Stylesheet {
    type Style: Default;

    fn active(&self) -> Appearance {
        Appearance::default()
    }

    fn hovered(&self) -> Appearance {
        Appearance {
            background: Background::Color(Color::TRANSPARENT),
            ..Appearance::default()
        }
    }
}
