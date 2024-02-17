#![allow(dead_code)]
use crate::theme::Theme;

pub type Renderer = iced::Renderer;
pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Theme, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Theme, Renderer>;
pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, Theme, Renderer>;
