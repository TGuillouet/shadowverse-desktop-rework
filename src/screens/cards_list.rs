use data::cards::{Card, CardClass};
use iced::{
    widget::{column, container, text},
    Length,
};
use widgets::table_row::TableRow;

use crate::widget::Element;

#[derive(Debug)]
pub enum Message {}

pub struct CardsList {
    cards: Vec<Card>,
}

impl CardsList {
    pub fn new() -> Self {
        Self {
            cards: vec![Card {
                id: "BP01-001".to_owned(),
                name: String::from("Test card"),
                card_class: CardClass::Swordcraft,
            }],
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let filters = cards_filters();
        let cards_list = cards_list(&self.cards);

        let card_row = TableRow::new(cards_list).row_height(40.0);

        container(card_row).into()
        // container(column![filters, card_row, cards_list].spacing(15))
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .align_x(iced::alignment::Horizontal::Center)
        //     .into()
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

fn cards_list<'a>(cards: &[Card]) -> Element<'a, Message> {
    let cards: Vec<Element<'a, Message>> = cards
        .iter()
        .map(|card| text(format!("{} - {}", card.id, card.name)).into())
        .collect();

    container(column(cards)).into()
}
