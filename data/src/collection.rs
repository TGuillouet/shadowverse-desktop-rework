use std::collections::HashSet;

use crate::cards::{Card, GameExtension};

pub struct ExtensionProgression {
    pub extension: GameExtension,
    pub extension_cards: Vec<Card>,
    pub owned_cards: HashSet<String, Card>,
}

impl ExtensionProgression {
    pub fn progression(&self) -> i32 {
        (self.owned_cards.len() / self.extension_cards.len()) as i32
    }
}
