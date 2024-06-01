use std::collections::HashMap;

use data::{
    cards::{Card, CardClass},
    collection::{CollectionCard, ExtensionProgression},
    config::Config,
    db::get_extension,
};
use iced::{
    widget::{column, combo_box, container, row, scrollable, text, text_input, Row, Svg},
    Command, Length,
};
use widgets::header::Column;
use widgets::table_row::TableRow;

use crate::{theme::Theme, widget::Element};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateQuantity(String, String),
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
    quantities: HashMap<String, String>,
}

impl CardsList {
    pub fn new(extension_progression: ExtensionProgression) -> Self {
        let mut quantities: HashMap<String, String> =
            HashMap::with_capacity(extension_progression.extension_cards.len());
        extension_progression
            .extension_cards
            .iter()
            .for_each(|extension_card| {
                let card = extension_card.card.clone();
                let quantity = extension_card.quantity.to_string();
                quantities.insert(card.id, quantity);
            });
        Self {
            columns: vec![
                Column::new("Owned").width(Length::FillPortion(1)),
                Column::new("Rarity").width(Length::FillPortion(2)),
                Column::new("Number").width(Length::FillPortion(2)),
                Column::new("Name").width(Length::FillPortion(4)),
                Column::new("Class").width(Length::FillPortion(2)),
                Column::new("Actions").width(Length::Fixed(100.0)),
            ],
            filtered_cards_list: extension_progression.clone().extension_cards,
            quantities,
            extension_progression,
            filter_name: String::new(),
            filter_cards_classes: combo_box::State::new(CardClass::ALL.to_vec()),
            filter_card_class: None,
        }
    }

    pub fn update(&mut self, config: &Config, message: Message) -> Command<Message> {
        match message {
            Message::UpdateQuantity(card_id, quantity) => {
                self.quantities.insert(card_id.clone(), quantity.clone());

                let Ok(quantity) = quantity.parse::<u8>() else {
                    return Command::none();
                };

                let _ = data::db::update_card_quantity(config, &card_id, quantity);

                self.extension_progression =
                    get_extension(config, &self.extension_progression.extension.id);
                self.filtered_cards_list
                    .clone_from(&self.extension_progression.extension_cards);
                self.filter_cards_list()
            }
            Message::Selected(card_class) => {
                self.filter_card_class = Some(card_class);

                self.filter_cards_list()
            }
            Message::FilterByName(card_name) => {
                self.filter_name = card_name;

                self.filter_cards_list()
            }
        };
        Command::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
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
        .height(Length::Fixed(70.0))
        .into();
        let cards_list = cards_list(&self.columns, &self.filtered_cards_list, &self.quantities);

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
                let name_contains_search = extension_card.card.name().contains(&self.filter_name);
                if let Some(class_search) = self.filter_card_class.as_ref() {
                    return name_contains_search && &extension_card.card.card_class == class_search;
                }
                name_contains_search
            })
            .collect();

        let mut quantities: HashMap<String, String> =
            HashMap::with_capacity(self.filtered_cards_list.len());
        self.filtered_cards_list.iter().for_each(|extension_card| {
            let card = extension_card.card.clone();
            let quantity = extension_card.quantity.to_string();
            quantities.insert(card.id, quantity);
        });
        self.quantities = quantities;
    }
}

fn cards_list<'a>(
    columns: &[Column],
    collection_cards: &'a [CollectionCard],
    quantities: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let headers = headers(columns);
    let card_rows: Vec<Element<'a, Message>> = collection_cards
        .iter()
        .map(|collection_card| {
            let quantity = quantities.get(&collection_card.card.id);
            let default_quantity = "0".to_string();
            table_row(
                &collection_card.card,
                collection_card.is_owned,
                quantity.unwrap_or(&default_quantity).clone(),
            )
            .into()
        })
        .collect();
    scrollable(
        column(vec![headers, column(card_rows).spacing(6.0).into()])
            .spacing(10.0)
            .padding(15.0),
    )
    .height(Length::Fill)
    .into()
}

fn headers<'a>(columns: &[Column]) -> Element<'a, Message> {
    let columns: Vec<Element<'a, Message>> = columns
        .iter()
        .map(|column| text(column.name.to_string()).width(column.width).into())
        .collect();
    row(columns).into()
}

fn table_row(
    card: &Card,
    is_owned: bool,
    quantity: String,
) -> TableRow<'_, Message, Theme, iced::Renderer> {
    let mut elements_row = Row::new().padding([0.0, 10.0]);

    let owned_graphic = if is_owned {
        Svg::new("resources/done.svg")
    } else {
        Svg::new("resources/close.svg")
    };
    elements_row = elements_row.push(
        owned_graphic
            .width(Length::FillPortion(1))
            .height(Length::Fill),
    );

    let card_rariry = text(card.rarity.clone())
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(card_rariry);

    let card_number = text(card.id.clone())
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(card_number);

    let card_name = text(card.name().clone())
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(card_name);

    let class = text(format!("{:?}", card.card_class))
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .vertical_alignment(iced::alignment::Vertical::Center);
    elements_row = elements_row.push(class);

    let quantity_input = text_input("", &quantity).on_input(|new_text| {
        let card_clone = card.clone();
        Message::UpdateQuantity(card_clone.id, new_text)
    });

    let actions_row = row![quantity_input].width(Length::Fixed(100.0));
    elements_row = elements_row.push(actions_row);

    TableRow::new(elements_row.align_items(iced::Alignment::Center)).row_height(35.0)
}
