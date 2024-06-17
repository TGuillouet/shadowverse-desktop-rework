use std::borrow::Cow;

use crate::cards::{Card, CardClass};

pub struct Deck<'a> {
    pub name: Cow<'a, str>,
    pub class: CardClass,
    pub deck_cards: Vec<DeckCard>,
}

pub struct DeckCard {
    pub card: Card,
    pub quantity: u8,
}
