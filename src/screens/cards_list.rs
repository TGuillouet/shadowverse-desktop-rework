use data::{
    cards::{Card, CardClass},
    collection::{CollectionCard, ExtensionProgression},
};
use iced::{
    widget::{button, column, combo_box, container, row, scrollable, text, text_input, Row},
    Command, Length,
};
use widgets::header::Column;
use widgets::table_row::TableRow;

use crate::{theme::Theme, widget::Element};

#[derive(Debug, Clone)]
pub enum Message {
    AddCard(Card),
    RemoveCard(Card),
    Selected(CardClass),
    FilterByName(String),
}

pub struct CardsList {
    columns: Vec<Column>,
    extension_progression: ExtensionProgression,

    filter_name: String,
    filter_cards_classes: iced::widget::combo_box::State<CardClass>,
    filter_card_class: Option<CardClass>,

    filtered_cards_list: Vec<CollectionCard>,
}

impl CardsList {
    pub fn new(extension_progression: ExtensionProgression) -> Self {
        Self {
            columns: vec![
                Column::new("Number").width(Length::FillPortion(1)),
                Column::new("Name").width(Length::FillPortion(2)),
                Column::new("Class").width(Length::FillPortion(1)),
                Column::new("Actions"),
            ],
            filtered_cards_list: extension_progression.clone().extension_cards,
            extension_progression,
            filter_name: String::new(),
            filter_cards_classes: combo_box::State::new(CardClass::ALL.to_vec()),
            filter_card_class: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddCard(card) => {
                println!("Add the card: {:?}", card);
            }
            Message::RemoveCard(card) => {
                println!("Remove the card: {:?}", card);
            }
            Message::Selected(card_class) => {
                println!("Selecting the card class: {:?}", card_class);
                self.filter_card_class = Some(card_class);

                self.filter_cards_list()
            }
            Message::FilterByName(card_name) => {
                println!("Name filter: {}", &card_name);
                self.filter_name = card_name;

                self.filter_cards_list()
            }
        };
        Command::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let filters = row![
            text_input("Type the card name here", &self.filter_name)
                .width(Length::FillPortion(3))
                .on_input(Message::FilterByName),
            combo_box(
                &self.filter_cards_classes,
                "Select the card class",
                self.filter_card_class.as_ref(),
                Message::Selected
            )
            .width(Length::FillPortion(1))
        ]
        .spacing(15.0)
        .padding(15.0)
        .into();
        let cards_list = cards_list(&self.columns, &self.filtered_cards_list);

        container(column(vec![filters, cards_list]))
            .max_width(800.0)
            .into()
    }

    fn filter_cards_list(&mut self) {
        self.filtered_cards_list = self
            .extension_progression
            .clone()
            .extension_cards
            .into_iter()
            .filter(|extension_card| {
                let name_contains_search = extension_card.card.name.contains(&self.filter_name);
                if let Some(class_search) = self.filter_card_class.as_ref() {
                    return name_contains_search && &extension_card.card.card_class == class_search;
                }
                name_contains_search
            })
            .collect();
    }
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
    scrollable(
        column(vec![headers, column(card_rows).spacing(6.0).into()])
            .spacing(10.0)
            .padding(15.0),
    )
    .into()
}

fn headers<'a>(columns: &Vec<Column>) -> Element<'a, Message> {
    let columns: Vec<Element<'a, Message>> = columns
        .iter()
        .map(|column| text(column.name.to_string()).width(column.width).into())
        .collect();
    row(columns).into()
}

fn table_row<'a>(card: &Card, is_owned: bool) -> TableRow<'a, Message, Theme, iced::Renderer> {
    let mut elements_row = Row::new().padding([0.0, 10.0]);
    let card_number = text(card.id.clone())
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(card_number);

    let card_name = text(card.name.clone())
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(card_name);

    let class = text(format!("{:?}", card.card_class))
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(class);

    let mut action_button = if !is_owned {
        button(text("Add")).on_press(Message::AddCard(card.clone()))
    } else {
        button(text("Remove")).on_press(Message::RemoveCard(card.clone()))
    };
    action_button = action_button.padding([0.0, 10.0]);
    let actions_row = row![action_button].width(Length::Fixed(150.0));
    elements_row = elements_row.push(actions_row);

    TableRow::new(elements_row.align_items(iced::Alignment::Center)).row_height(35.0)
}
