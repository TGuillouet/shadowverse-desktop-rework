use data::cards::{Card, CardClass, GameExtension};
use iced::{
    widget::{column, container, row, text},
    Length,
};
use widgets::header::Column;
use widgets::table_row::TableRow;

use crate::{theme::Theme, widget::Element};

#[derive(Debug)]
pub enum Message {}

pub struct CardsList {
    columns: Vec<Column>,
    cards: Vec<Card>,
}

impl CardsList {
    pub fn new() -> Self {
        Self {
            columns: vec![Column::new("Name"), Column::new("Class")],
            cards: vec![
                Card {
                    id: "BP01-001".to_owned(),
                    name: String::from("Test card"),
                    card_class: CardClass::Swordcraft,
                    extension: GameExtension {
                        id: "BT01".to_string(),
                        name: "Advent of genesis".to_string(),
                    },
                },
                Card {
                    id: "BP01-002".to_owned(),
                    name: String::from("Test card 2"),
                    card_class: CardClass::Runecraft,
                    extension: GameExtension {
                        id: "BT01".to_string(),
                        name: "Advent of genesis".to_string(),
                    },
                },
            ],
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let filters = cards_filters();
        let cards_list = cards_list(&self.columns, &self.cards);

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

fn cards_list<'a>(columns: &Vec<Column>, cards: &[Card]) -> Element<'a, Message> {
    let headers = headers(columns);
    let card_rows: Vec<Element<'a, Message>> =
        cards.iter().map(|card| table_row(card).into()).collect();
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

fn table_row<'a>(card: &Card) -> TableRow<'a, Message, Theme, iced::Renderer> {
    let card_name = text(card.name.clone())
        .width(Length::Fixed(150.0))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);

    let class = text(format!("{:?}", card.card_class))
        .width(Length::Fixed(150.0))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);

    TableRow::new(row![card_name, class]).row_height(35.0)
}
