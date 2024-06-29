use std::borrow::Cow;

use crate::cards::{Card, CardClass};

pub struct Deck<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub class: CardClass,
    pub deck_cards: Vec<DeckCard<'a>>,
}

pub struct DeckCard<'a> {
    pub id: Cow<'a, str>,
    pub card: Card,
    pub quantity: u8,
}
