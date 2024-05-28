use iced::widget::text;

use super::Theme;

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
