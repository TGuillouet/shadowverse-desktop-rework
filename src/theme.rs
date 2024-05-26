use iced::{
    application::{self},
    color, overlay,
    widget::{
        button, container, progress_bar,
        scrollable::{self, Scroller},
        svg, text, text_input,
    },
    Border, Color, Vector,
};
use widgets::table_row;

#[derive(Debug, Clone)]
struct Palette {
    primary: Color,
    dark_primary: Color,
    light_primary: Color,
    accent: Color,
    background: Color,
    background_light: Color,
    primary_text: Color,
    secondary_text: Color,
}

#[derive(Debug, Clone)]
pub struct Theme {
    palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            palette: Palette {
                primary: color!(0x00, 0xbc, 0xd4),
                dark_primary: color!(0x00, 0x97, 0xa7),
                light_primary: color!(0xb2, 0xeb, 0xf2),
                accent: color!(0x00, 0x74, 0x91),
                background: color!(0x27, 0x29, 0x2d),
                background_light: color!(0x3a, 0x3d, 0x42),
                primary_text: color!(0xf9, 0xf9, 0xf9),
                secondary_text: color!(0x75, 0x75, 0x75),
            },
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette.background,
            text_color: self.palette.primary_text,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Sidebar,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Sidebar => container::Appearance {
                text_color: Color::WHITE.into(),
                background: Some(self.palette.background_light.into()),
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
        }
    }
}

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
                // border: Border {
                //     radius: 4.0.into(),
                //     width: 1.0,
                //     color: self.palette.light_primary,
                // },
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

#[derive(Debug, Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => text::Appearance {
                ..Default::default()
            },
        }
    }
}

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
            background: Color::WHITE.into(),
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

#[derive(Debug, Default, Clone, Copy)]
pub enum ProgressBar {
    #[default]
    Default,
}

impl progress_bar::StyleSheet for Theme {
    type Style = ProgressBar;

    fn appearance(&self, _style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: self.palette.light_primary.into(),
            bar: self.palette.primary.into(),
            border_radius: 15.0.into(),
        }
    }
}

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

// Combo box
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

// SVG
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
