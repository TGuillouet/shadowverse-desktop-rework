use data::{
    // cards::{Card, CardClass, GameExtension},
    // collection::{CollectionCard, ExtensionProgression},
    config::Config,
    db::get_extensions,
};
use iced::{
    widget::{button, container, row, text},
    Application, Command, Length,
};

use crate::screens;

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    ExtensionsList(screens::extensions_list::Message),
    CardsList(screens::cards_list::Message),
}

pub enum AppScreens {
    Extensions(screens::extensions_list::ExtensionsList),
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
        let progression = get_extensions();
        let application = Self {
            config: flags,
            screen: AppScreens::CardsList(screens::cards_list::CardsList::new(
                progression[0].clone(),
            )),
            // screen: AppScreens::Extensions(screens::extensions_list::ExtensionsList::new(
            //     progression,
            // )),
        };
        (application, Command::none())
    }

    fn title(&self) -> String {
        "Iced test architecture".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            ApplicationMessage::ExtensionsList(message) => {
                match message {
                    screens::extensions_list::Message::ToDetails(extension_progression) => {
                        println!("To details for {:?}", extension_progression);
                    }
                }
                Command::none()
            }
            ApplicationMessage::CardsList(message) => {
                let AppScreens::CardsList(screen) = &mut self.screen else {
                    return Command::none();
                };

                screen.update(message).map(ApplicationMessage::CardsList)
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let screen = match &self.screen {
            AppScreens::Extensions(screen) => screen.view().map(ApplicationMessage::ExtensionsList),
            AppScreens::CardsList(screen) => screen.view().map(ApplicationMessage::CardsList),
        };

        container(screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    }
}
