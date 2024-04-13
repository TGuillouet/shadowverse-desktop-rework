use std::sync::Arc;

use data::{
    collection::ExtensionProgression,
    config::Config,
    db::{self, get_extensions},
};
use iced::{
    widget::{container, Row},
    Application, Command, Length, Subscription,
};

use crate::{screens, widgets::sidebar::sidebar};

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    CardsListUpdater(screens::update::Message),
    ExtensionsList(screens::extensions_list::Message),
    CardsList(screens::cards_list::Message),
    OnSidebarClick(String),
}

pub enum AppScreens {
    CardsListUpdater(screens::update::CardsUpdater),
    Extensions(screens::extensions_list::ExtensionsList),
    CardsList(screens::cards_list::CardsList),
}

pub struct IcedApplication {
    config: Arc<Config>,
    screen: AppScreens,
}

impl IcedApplication {
    fn navigate_to_extensions(&mut self) {
        let progression = get_extensions(&self.config);
        self.screen =
            AppScreens::Extensions(screens::extensions_list::ExtensionsList::new(progression));
    }

    fn navigate_to_progress(&mut self, extension_progression: &ExtensionProgression) {
        self.screen = AppScreens::CardsList(screens::cards_list::CardsList::new(
            extension_progression.clone(),
        ))
    }
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = ApplicationMessage;
    type Theme = crate::theme::Theme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let application = Self {
            config: Arc::new(flags),
            screen: AppScreens::CardsListUpdater(screens::update::CardsUpdater::new()),
        };
        (application, Command::none())
    }

    fn title(&self) -> String {
        "Shadowverse utils".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            ApplicationMessage::CardsListUpdater(message) => {
                let AppScreens::CardsListUpdater(screen) = &mut self.screen else {
                    return Command::none();
                };
                let command = screen
                    .update(&self.config, message.clone())
                    .map(ApplicationMessage::CardsListUpdater);

                match &message {
                    screens::update::Message::CardFetched(event) => match event {
                        screens::update::Event::Finished => {
                            self.navigate_to_extensions();
                            return Command::none();
                        }
                        _ => {}
                    },
                    screens::update::Message::MetadatasLoaded(metadatas_result) => {
                        let Ok((total_cards, _)) = metadatas_result else {
                            return Command::none();
                        };

                        let already_present_in_db = db::get_all_cards_number(&self.config);
                        if already_present_in_db.len() == total_cards.clone() as usize {
                            self.navigate_to_extensions();
                        }
                        return Command::none();
                    }
                    _ => {}
                };

                command
            }
            ApplicationMessage::ExtensionsList(message) => {
                match message {
                    screens::extensions_list::Message::ToDetails(extension_progression) => {
                        self.navigate_to_progress(&extension_progression);
                    }
                }
                Command::none()
            }
            ApplicationMessage::CardsList(message) => {
                let AppScreens::CardsList(screen) = &mut self.screen else {
                    return Command::none();
                };

                screen
                    .update(&self.config, message)
                    .map(ApplicationMessage::CardsList)
            }
            ApplicationMessage::OnSidebarClick(screen_key) => {
                match screen_key.as_str() {
                    "progressions" => self.navigate_to_extensions(),
                    _ => self.navigate_to_extensions(),
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let screen = match &self.screen {
            AppScreens::CardsListUpdater(screen) => {
                screen.view().map(ApplicationMessage::CardsListUpdater)
            }
            AppScreens::Extensions(screen) => screen.view().map(ApplicationMessage::ExtensionsList),
            AppScreens::CardsList(screen) => screen.view().map(ApplicationMessage::CardsList),
        };

        let sidebar_option = match &self.screen {
            AppScreens::CardsListUpdater(_) => None,
            _ => Some(sidebar()),
        };

        let app_row = Row::new()
            .push_maybe(sidebar_option)
            .push(container(screen).align_x(iced::alignment::Horizontal::Center));

        container(app_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match &self.screen {
            AppScreens::CardsListUpdater(screen) => screen
                .subscription(self.config.clone())
                .map(ApplicationMessage::CardsListUpdater),
            _ => Subscription::none(),
        }
    }
}
