use std::path::PathBuf;

use cards_updater::{get_cards, get_max_page, get_number_of_cards};
use data::{cards::Card, config::Config, db};
use iced::{
    futures::SinkExt,
    subscription,
    widget::{column, container, progress_bar, text},
    Command, Length, Subscription,
};

use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {
    MetadatasLoaded((u32, u32)),
    CardsListLoaded(Vec<String>),
    // CardFetched(Card),
    CardFetched(Event),
}

#[derive(Debug, Clone)]
pub struct CardsUpdater {
    current_card_index: usize,
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
                self.total_cards = number_of_cards;
                self.number_of_pages = number_of_pages;

                self.step = DownloadStep::CardsList { number_of_pages: 1 };
            }
            Message::CardsListLoaded(cards_list) => {
                self.cards_list = cards_list.clone();
                self.step = DownloadStep::Card(cards_list.clone());
            }
            Message::CardFetched(event) => match event {
                Event::Card(card) => {
                    let _ = db::upsert_card(&config, card.clone());

                    self.current_card_index += 1;
                    self.current_card_name = card.name;
                }
                Event::Finished => {
                    self.step = DownloadStep::Finished;
                }
            },
        }

        Command::none()
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let screen = match &self.step {
            DownloadStep::Metadatas => text("Fetching the number of available cards").into(),
            DownloadStep::CardsList { number_of_pages: _ } => {
                text("Extracting the list of cards").into()
            }
            DownloadStep::Card(_) => self.card_sync_view(),
            DownloadStep::Finished => text("Finished").into(),
        };

        container(screen)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn card_sync_view<'a>(&self) -> Element<'a, Message> {
        column![
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
        .spacing(10.0)
        .into()
    }

    pub fn subscription(&self, config: &Config) -> iced::Subscription<Message> {
        match self.step.clone() {
            DownloadStep::Metadatas => fetch_metadata().map(Message::MetadatasLoaded),
            DownloadStep::CardsList { number_of_pages } => {
                fetch_cards_list(number_of_pages).map(Message::CardsListLoaded)
            }
            DownloadStep::Card(cards_list) => {
                fetch_single_card(cards_list.clone(), config.covers_directory.clone())
                    .map(Message::CardFetched)
            }
            _ => Subscription::none(),
        }
    }
}

#[derive(Debug, Clone)]
enum DownloadStep {
    Metadatas,
    CardsList { number_of_pages: u32 },
    Card(Vec<String>),
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

#[derive(Debug, Clone)]
pub enum Event {
    Card(Card),
    Finished,
}

fn fetch_single_card(cards: Vec<String>, covers_path: PathBuf) -> iced::Subscription<Event> {
    struct DownloadCardsTask;

    subscription::channel(
        std::any::TypeId::of::<DownloadCardsTask>(),
        0,
        move |mut output| async move {
            let mut cards = cards.iter().clone();

            while let Some(current_card) = cards.next() {
                let card = cards_updater::download_card(&current_card, &covers_path);
                let _ = output.send(Event::Card(card)).await;
            }

            loop {
                let _ = output.send(Event::Finished).await;
            }
        },
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
