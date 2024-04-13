use std::{collections::HashSet, sync::Arc};

use cards_updater::{get_cards, get_max_page, get_number_of_cards};
use data::{cards::Card, config::Config, db};
use iced::{
    futures::SinkExt,
    subscription,
    widget::{column, container, progress_bar, text},
    Command, Length,
};

use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {
    CardFetched(Event),
}

#[derive(Debug, Clone)]
pub struct CardsUpdater {
    current_card_index: usize,
    current_card_name: String,

    total_cards: u32,

    step: DownloadStep,
}

impl CardsUpdater {
    pub fn new() -> Self {
        Self {
            current_card_index: 0,
            current_card_name: "".to_string(),

            total_cards: 0,

            step: DownloadStep::Metadatas,
        }
    }

    fn progress(&self) -> f32 {
        self.current_card_index as f32 / self.total_cards as f32
    }

    pub fn update(&mut self, config: &Config, message: Message) -> Command<Message> {
        match message {
            Message::CardFetched(event) => match event {
                Event::MetadatasList(total_cards) => {
                    self.total_cards = total_cards;
                    self.step = DownloadStep::Card;
                }
                Event::IncreaseDownloadedCounter(increment) => {
                    self.current_card_index += increment;
                }
                Event::Card(card) => {
                    let _ = db::upsert_card(&config, card.clone());

                    self.current_card_index += 1;
                    self.current_card_name = card.name;
                }
                Event::Finished => {}
                Event::Error(error) => {
                    println!("{:?}", error);
                }
            },
        }

        Command::none()
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let screen = match &self.step {
            DownloadStep::Metadatas => text("Loading the metadatas").into(),
            DownloadStep::Card => self.card_sync_view(),
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

    pub fn subscription(&self, config: Arc<Config>) -> iced::Subscription<Message> {
        fetch_single_card(config.clone()).map(Message::CardFetched)
    }
}

#[derive(Debug, Clone)]
enum DownloadStep {
    Metadatas,
    Card,
}

#[derive(Debug, Clone)]
pub enum Event {
    MetadatasList(u32),
    IncreaseDownloadedCounter(usize),
    Card(Card),
    Error(cards_updater::ErrorKind),
    Finished,
}

fn fetch_single_card(config: Arc<Config>) -> iced::Subscription<Event> {
    struct DownloadCardsTask;

    subscription::channel(
        std::any::TypeId::of::<DownloadCardsTask>(),
        0,
        move |mut output| async move {
            let Ok(number_of_cards) = get_number_of_cards().await else {
                let _ = output
                    .send(Event::Error(cards_updater::ErrorKind::NumberOfCardsError))
                    .await;
                return iced::futures::future::pending().await;
            };

            let _ = output.send(Event::MetadatasList(number_of_cards)).await;

            let max_page = get_max_page().await;

            for page_number in 1..=max_page {
                let cards = get_cards(page_number).await.unwrap();
                let cards_to_download = exclude_already_downloaded(cards.clone(), &config);

                let _ = output
                    .send(Event::IncreaseDownloadedCounter(
                        cards.len() - cards_to_download.len(),
                    ))
                    .await;

                let mut cards_iter = cards_to_download.iter();
                while let Some(current_card) = cards_iter.next() {
                    match cards_updater::download_card(&current_card, &config.covers_directory) {
                        Ok(card) => {
                            let _ = output.send(Event::Card(card)).await;
                        }
                        Err(error) => {
                            let _ = output.send(Event::Error(error)).await;
                        }
                    };
                }
            }

            let _ = output.send(Event::Finished).await;
            iced::futures::future::pending().await
        },
    )
}

fn exclude_already_downloaded(cards_list: Vec<String>, config: &Config) -> Vec<String> {
    let already_downloaded: Vec<String> = db::get_all_cards_number(config);
    let item_set: HashSet<String> = already_downloaded.into_iter().collect();
    cards_list
        .into_iter()
        .filter(|item| !item_set.contains(item))
        .collect()
}
