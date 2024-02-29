use iced::{
    widget::{button, column, container, text},
    Length,
};

use crate::{
    app::ApplicationMessage,
    theme,
    widget::{Button, Container},
};

pub fn sidebar<'a>() -> Container<'a, ApplicationMessage> {
    let buttons_container =
        container(column![sidebar_button("Progression", "progression"),]).height(Length::Fill);

    container(buttons_container)
        .style(theme::Container::Sidebar)
        .padding(15.0)
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
}

fn sidebar_button<'a>(label: &'a str, key: &str) -> Button<'a, ApplicationMessage> {
    button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fixed(30.0))
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
    .padding(5)
    .width(Length::Fill)
    .on_press(ApplicationMessage::OnSidebarClick(key.to_owned()))
    .style(theme::Button::Sidebar)
}
