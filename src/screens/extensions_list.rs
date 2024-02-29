use data::collection::ExtensionProgression;
use iced::{
    widget::{button, column, container, progress_bar, row, scrollable, text},
    Length,
};

use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {
    ToDetails(ExtensionProgression),
}

pub struct ExtensionsList {
    extensions_progress: Vec<ExtensionProgression>,
}

impl ExtensionsList {
    pub fn new(progressions: Vec<ExtensionProgression>) -> Self {
        Self {
            extensions_progress: progressions,
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let extensions_widgets: Vec<Element<'a, Message>> = self
            .extensions_progress
            .clone()
            .into_iter()
            .map(|progress| extension_progress(&progress))
            .collect();

        container(scrollable(column(extensions_widgets)))
            .padding([15.0, 0.0])
            .max_width(800.0)
            .into()
    }
}

fn extension_progress<'a>(progress: &ExtensionProgression) -> Element<'a, Message> {
    let label_text = text(format!(
        "{} - {}",
        progress.extension.id,
        progress.extension.name.clone()
    ))
    .width(Length::FillPortion(4))
    .height(Length::Fill)
    .vertical_alignment(iced::alignment::Vertical::Center);

    let progress_text = text(format!(
        "{} / {} ({:.2}%)",
        progress.owned_cards().len(),
        progress.cards().len(),
        progress.progression() * 100.0
    ))
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .horizontal_alignment(iced::alignment::Horizontal::Right)
    .vertical_alignment(iced::alignment::Vertical::Center);

    let progress_bar = progress_bar(0.0..=1.0, progress.progression())
        .height(Length::Fixed(15.0))
        .width(Length::FillPortion(3));

    let progress_row = row![progress_bar, progress_text]
        .align_items(iced::Alignment::Center)
        .width(Length::FillPortion(6));

    button(row![label_text, progress_row])
        .padding([0.0, 25.0])
        .width(Length::Fill)
        .height(Length::Fixed(60.0))
        .style(crate::theme::Button::ExtensionButton)
        .on_press(Message::ToDetails(progress.clone()))
        .into()
}
