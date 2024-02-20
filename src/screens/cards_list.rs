use data::{
    cards::{Card, CardClass, GameExtension},
    collection::{CollectionCard, ExtensionProgression},
};
use iced::{
    widget::{button, column, container, row, text, Row},
    Length,
};
use widgets::header::Column;
use widgets::table_row::TableRow;

use crate::{theme::Theme, widget::Element};

#[derive(Debug, Clone)]
pub enum Message {}

pub struct CardsList {
    columns: Vec<Column>,
    extension_progression: ExtensionProgression,
}

impl CardsList {
    pub fn new(extension_progression: ExtensionProgression) -> Self {
        Self {
            columns: vec![
                Column::new("Name"),
                Column::new("Class"),
                Column::new("Actions"),
            ],
            extension_progression,
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let filters = cards_filters();
        let cards_list = cards_list(&self.columns, &self.extension_progression.extension_cards);

        container(column(vec![filters, cards_list])).into()
    }
}

fn cards_filters<'a>() -> Element<'a, Message> {
    container(
        text("Filters")
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center),
    )
    .width(Length::Fill)
    .into()
}

fn cards_list<'a>(
    columns: &Vec<Column>,
    collection_cards: &Vec<CollectionCard>,
) -> Element<'a, Message> {
    let headers = headers(columns);
    let card_rows: Vec<Element<'a, Message>> = collection_cards
        .iter()
        .map(|collection_card| table_row(&collection_card.card, collection_card.is_owned).into())
        .collect();
    column(vec![headers, column(card_rows).spacing(6.0).into()])
        .spacing(10.0)
        .padding(15.0)
        .into()
}

fn headers<'a>(columns: &Vec<Column>) -> Element<'a, Message> {
    let columns: Vec<Element<'a, Message>> = columns
        .iter()
        .map(|column| {
            text(column.name.to_string())
                .width(Length::Fixed(column.width))
                .into()
        })
        .collect();
    row(columns).into()
}

fn table_row<'a>(card: &Card, is_owned: bool) -> TableRow<'a, Message, Theme, iced::Renderer> {
    let mut elements_row = Row::new();
    let card_name = text(card.name.clone())
        .width(Length::Fixed(150.0))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(card_name);

    let class = text(format!("{:?}", card.card_class))
        .width(Length::Fixed(150.0))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(class);

    let action_button = if !is_owned {
        button(text("Add"))
    } else {
        button(text("Remove"))
    };
    elements_row = elements_row.push(action_button);

    TableRow::new(elements_row.align_items(iced::Alignment::Center)).row_height(35.0)
}
