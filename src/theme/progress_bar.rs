use iced::widget::progress_bar;

use super::Theme;

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
