use crate::{
    cards::{Card, CardClass, GameExtension},
    collection::{CollectionCard, ExtensionProgression},
};

pub fn get_extensions() -> Vec<ExtensionProgression> {
    vec![ExtensionProgression {
        extension: GameExtension {
            id: "BP01".to_string(),
            name: "Advent of genesis".to_string(),
        },
        extension_cards: vec![
            CollectionCard {
                card: Card {
                    id: "BP01-001".to_owned(),
                    name: String::from("Test card"),
                    card_class: CardClass::Swordcraft,
                    extension: GameExtension {
                        id: "BT01".to_string(),
                        name: "Advent of genesis".to_string(),
                    },
                },
                is_owned: false,
            },
            CollectionCard {
                card: Card {
                    id: "BP01-002".to_owned(),
                    name: String::from("Test card 2"),
                    card_class: CardClass::Runecraft,
                    extension: GameExtension {
                        id: "BT01".to_string(),
                        name: "Advent of genesis".to_string(),
                    },
                },
                is_owned: true,
            },
        ],
    }]
}
