use crate::cards::{Card, GameExtension};

#[derive(Debug, Clone)]
pub struct ExtensionProgression {
    pub extension: GameExtension,
    pub extension_cards: Vec<CollectionCard>,
}

impl ExtensionProgression {
    pub fn owned_cards(&self) -> Vec<&Card> {
        self.extension_cards
            .iter()
            .filter(|item| item.is_owned)
            .map(|item| &item.card)
            .collect()
    }

    pub fn cards(&self) -> Vec<&Card> {
        self.extension_cards
            .iter()
            .map(|extension_card| &extension_card.card)
            .collect()
    }

    pub fn progression(&self) -> f32 {
        let owned_cards_length = self.owned_cards().len() as f32;
        owned_cards_length / self.extension_cards.len() as f32
    }
}

#[derive(Debug, Clone)]
pub struct CollectionCard {
    pub card: Card,
    pub is_owned: bool,
}
