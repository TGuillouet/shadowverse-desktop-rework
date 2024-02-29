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
    let top_container = container(column![]).height(Length::FillPortion(3));
    let middle_container = container(column![sidebar_button("Progression", "progression"),])
        .height(Length::FillPortion(5))
        .align_y(iced::alignment::Vertical::Center);
    let bottom_container = container(column![]).height(Length::FillPortion(3));
    container(
        column![top_container, middle_container, bottom_container]
            .padding(10)
            .spacing(10),
    )
    .style(theme::Container::Sidebar)
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
