use iced::{
    widget::{
        container,
        scrollable::{self, Scroller},
    },
    Border, Color,
};

use super::Theme;

impl scrollable::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            container: container::Appearance::default(),
            scrollbar: scrollable::Scrollbar {
                background: None,
                border: Border {
                    color: Color::BLACK,
                    radius: 5.0.into(),
                    width: 0.0,
                },
                scroller: Scroller {
                    color: self.palette.primary,
                    border: Border {
                        color: self.palette.light_primary,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        scrollable::Appearance {
            container: container::Appearance::default(),
            scrollbar: scrollable::Scrollbar {
                background: None,
                border: Border {
                    color: Color::BLACK,
                    radius: 5.0.into(),
                    width: 0.0,
                },
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar {
                        self.palette.dark_primary
                    } else {
                        self.palette.primary
                    },
                    border: Border {
                        color: self.palette.dark_primary,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                },
            },
            gap: None,
        }
    }
}
