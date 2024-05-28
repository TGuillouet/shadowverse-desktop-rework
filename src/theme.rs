use iced::{color, Color};

mod application;
mod button;
mod combo_box;
mod container;
mod progress_bar;
mod scrollable;
mod svg;
mod table_row;
mod text;
mod text_input;

pub use button::Button;
pub use container::Container;

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
