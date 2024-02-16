use data::config::Config;
use iced::{
    widget::{container, text},
    Application, Command, Length,
};

#[derive(Debug)]
pub enum ApplicationMessage {}

pub struct IcedApplication {
    config: Config,
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = ApplicationMessage;
    type Theme = crate::theme::Theme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let application = Self { config: flags };
        (application, Command::none())
    }

    fn title(&self) -> String {
        "Iced test architecture".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        container(text("Test"))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
