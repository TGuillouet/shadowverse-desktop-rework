use crate::{
    cards::{Card, CardClass, GameExtension},
    collection::{CollectionCard, ExtensionProgression},
    config::Config,
    environment::config_directory,
};
use rusqlite::Connection;

pub fn setup_db(config: &Config) {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    // TODO: Use execute_batch to create a transaction
    let extension_table_result = connection.execute(
        "CREATE TABLE IF NOT EXISTS extension (
            id VARCHAR(50) PRIMARY KEY,
            name VARCHAR(100) NOT NULL
        )",
        (),
    );

    let cards_table_result = connection.execute(
        "CREATE TABLE IF NOT EXISTS card (
            id TEXT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            card_class VARCHAR(50) NOT NULL,
            extension_id VARCHAR(50),
            FOREIGN KEY (extension_id) REFERENCES extension (id)
        )",
        (),
    );

    let collected_cards = connection.execute(
        "CREATE TABLE IF NOT EXISTS collected_cards (
            card_id VARCHAR(50),
            is_owned INTEGER,
            FOREIGN KEY (card_id) REFERENCES card (id)
        )",
        (),
    );
}

pub fn get_extensions() -> Vec<ExtensionProgression> {
    vec![
        ExtensionProgression {
            extension: GameExtension {
                id: "BP02".to_string(),
                name: "Rage of bahamut".to_string(),
            },
            extension_cards: vec![
                CollectionCard {
                    card: Card {
                        id: "BP02-001".to_owned(),
                        name: String::from("Test card"),
                        card_class: CardClass::Swordcraft,
                        extension: GameExtension {
                            id: "BT02".to_string(),
                            name: "Rage of bahamut".to_string(),
                        },
                    },
                    is_owned: false,
                },
                CollectionCard {
                    card: Card {
                        id: "BP02-002".to_owned(),
                        name: String::from("Test card 2"),
                        card_class: CardClass::Runecraft,
                        extension: GameExtension {
                            id: "BT02".to_string(),
                            name: "Rage of bahamut".to_string(),
                        },
                    },
                    is_owned: true,
                },
            ],
        },
        ExtensionProgression {
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
        },
    ]
}
