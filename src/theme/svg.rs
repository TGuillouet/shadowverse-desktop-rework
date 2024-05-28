use iced::{widget::svg, Color};

use super::Theme;

impl svg::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(Color::WHITE),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(Color::WHITE),
        }
    }
}
