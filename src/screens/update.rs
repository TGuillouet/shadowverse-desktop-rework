use std::path::PathBuf;

use cards_updater::{get_cards, get_max_page, get_number_of_cards};
use data::{cards::Card, config::Config};
use iced::{
    subscription,
    widget::{column, container, progress_bar, text},
    Command, Length, Subscription,
};

use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {
    MetadatasLoaded((u32, u32)),
    CardsListLoaded(Vec<String>),
    CardFetched(Card),
}

#[derive(Debug, Clone)]
pub struct CardsUpdater {
    current_card_index: u32,
    current_card_name: String,

    total_cards: u32,
    number_of_pages: u32,
    cards_list: Vec<String>,

    step: DownloadStep,
}

impl CardsUpdater {
    pub fn new() -> Self {
        Self {
            current_card_index: 0,
            current_card_name: "".to_string(),

            total_cards: 0,
            number_of_pages: 0,
            cards_list: Vec::new(),

            step: DownloadStep::Metadatas,
        }
    }

    fn progress(&self) -> f32 {
        self.current_card_index as f32 / self.total_cards as f32
    }

    pub fn update(&mut self, config: &Config, message: Message) -> Command<Message> {
        match message {
            Message::MetadatasLoaded((number_of_cards, number_of_pages)) => {
                // self.total_cards = number_of_cards;
                self.total_cards = 2;
                self.number_of_pages = number_of_pages;

                self.step = DownloadStep::CardsList { number_of_pages };
            }
            Message::CardsListLoaded(cards_list) => {
                // Add the tasks for each card
                self.cards_list = cards_list.clone();
                self.step = DownloadStep::Card(cards_list[0].clone());
            }
            Message::CardFetched(card) => {
                // Insert the card in the database
                println!("{:?}", card);
                self.current_card_index += 1;

                if self.current_card_index == self.total_cards - 1 {
                    self.step = DownloadStep::Finished;
                }
            }
        }

        Command::none()
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        // TODO: Screen for when we fetch the cards numbers list (Step Fetch cards list)

        // Downloading cards screen
        let update_in_progress_column = column![
            text(format!(
                "Syncing the cards list: {} / {}",
                self.current_card_index, self.total_cards
            ))
            .width(Length::Fixed(300.0))
            .horizontal_alignment(iced::alignment::Horizontal::Center),
            progress_bar(0.0..=1.0, self.progress())
                .width(300)
                .height(15),
            text(&self.current_card_name)
                .width(Length::Fixed(300.0))
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(10.0);

        container(update_in_progress_column)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self, config: &Config) -> iced::Subscription<Message> {
        match self.step.clone() {
            DownloadStep::Metadatas => fetch_metadata().map(Message::MetadatasLoaded),
            DownloadStep::CardsList { number_of_pages } => {
                fetch_cards_list(number_of_pages).map(Message::CardsListLoaded)
            }
            DownloadStep::Card(card) => {
                fetch_single_card(card, &config.covers_directory).map(Message::CardFetched)
            }
            _ => Subscription::none(),
        }
    }
}

#[derive(Debug, Clone)]
enum DownloadStep {
    Metadatas,
    CardsList { number_of_pages: u32 },
    Card(String),
    Finished,
}

#[derive(Debug)]
enum State {
    CardsListFetchReady,
    CardsListFetchFinished,
}

enum MetadataState {
    FetchNumberOfCards,
    MetadatasFetched,
}

enum SingleCard {
    Ready { card_number: String },
    Finished,
}

fn fetch_metadata() -> iced::Subscription<(u32, u32)> {
    subscription::unfold(
        "list_metadata_task",
        MetadataState::FetchNumberOfCards,
        |state| fetch_metadatas_task(state),
    )
}

fn fetch_cards_list(page_number: u32) -> iced::Subscription<Vec<String>> {
    subscription::unfold(
        "cards_list_task",
        State::CardsListFetchReady,
        move |state| fetch_cards_list_task(page_number, state),
    )
}

fn fetch_single_card<'a>(card_number: String, covers_path: &PathBuf) -> iced::Subscription<Card> {
    let card = card_number.clone();
    subscription::unfold(
        "card_task",
        SingleCard::Ready { card_number: card },
        |state| fetch_single_card_task(covers_path, state),
    )
}

async fn fetch_metadatas_task(state: MetadataState) -> ((u32, u32), MetadataState) {
    match state {
        MetadataState::FetchNumberOfCards => {
            let number_of_cards = get_number_of_cards().await.unwrap();
            let max_page = get_max_page().await;
            ((number_of_cards, max_page), MetadataState::MetadatasFetched)
        }
        MetadataState::MetadatasFetched => iced::futures::future::pending().await,
    }
}

async fn fetch_cards_list_task(max_page_number: u32, state: State) -> (Vec<String>, State) {
    match state {
        State::CardsListFetchReady => {
            let mut all_cards = Vec::new();
            for page_number in 1..=max_page_number {
                let cards_list = get_cards(page_number).await;
                all_cards.extend(cards_list.into_iter());
            }
            (all_cards, State::CardsListFetchFinished)
        }
        State::CardsListFetchFinished => iced::futures::future::pending().await,
    }
}

async fn fetch_single_card_task(
    // card_number: String,
    cover_directory: &PathBuf,
    state: SingleCard,
) -> (Card, SingleCard) {
    match state {
        SingleCard::Ready { card_number } => {
            // let card_number = card_number.clone();
            let card = cards_updater::download_card(&card_number, cover_directory).await;
            (card, SingleCard::Finished)
        }
        SingleCard::Finished => iced::futures::future::pending().await,
    }
}
