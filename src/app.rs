use data::{
    cards::{Card, CardClass, GameExtension},
    collection::{CollectionCard, ExtensionProgression},
    config::Config,
};
use iced::{widget::container, Application, Command, Length};

use crate::screens;

#[derive(Debug)]
pub enum ApplicationMessage {
    CardsList(screens::cards_list::Message),
}

pub enum AppScreens {
    CardsList(screens::cards_list::CardsList),
}

pub struct IcedApplication {
    config: Config,
    screen: AppScreens,
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = ApplicationMessage;
    type Theme = crate::theme::Theme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let progression = ExtensionProgression {
            extension: GameExtension {
                id: "BP01".to_string(),
                name: "Advent of genesis".to_string(),
            },
            extension_cards: vec![
                CollectionCard {
                    card: Card {
                        id: "BP01-001".to_owned(),
                        name: String::from("Test card"),
                        card_class: CardClass::Swordcraft,
                        extension: GameExtension {
                            id: "BT01".to_string(),
                            name: "Advent of genesis".to_string(),
                        },
                    },
                    is_owned: false,
                },
                CollectionCard {
                    card: Card {
                        id: "BP01-002".to_owned(),
                        name: String::from("Test card 2"),
                        card_class: CardClass::Runecraft,
                        extension: GameExtension {
                            id: "BT01".to_string(),
                            name: "Advent of genesis".to_string(),
                        },
                    },
                    is_owned: true,
                },
            ],
        };

        let application = Self {
            config: flags,
            screen: AppScreens::CardsList(screens::cards_list::CardsList::new(progression)),
        };
        (application, Command::none())
    }

    fn title(&self) -> String {
        "Iced test architecture".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let screen = match &self.screen {
            AppScreens::CardsList(screen) => screen.view().map(ApplicationMessage::CardsList),
        };

        container(screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
