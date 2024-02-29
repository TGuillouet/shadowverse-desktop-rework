use crate::{
    cards::{Card, CardClass, GameExtension},
    collection::{CollectionCard, ExtensionProgression},
    config::Config,
};
use rusqlite::Connection;

pub fn setup_db(config: &Config) -> Result<(), ()> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    let db_setup_result = connection.execute_batch(
        "BEGIN;
        CREATE TABLE IF NOT EXISTS extension (
            id VARCHAR(50) PRIMARY KEY,
            name VARCHAR(100) NOT NULL
        );
        CREATE TABLE IF NOT EXISTS card (
            id TEXT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            card_class VARCHAR(50) NOT NULL,
            extension_id VARCHAR(50),
            FOREIGN KEY (extension_id) REFERENCES extension (id)
        );
        CREATE TABLE IF NOT EXISTS collected_cards (
            card_id VARCHAR(50),
            is_owned INTEGER,
            FOREIGN KEY (card_id) REFERENCES card (id)
        );
        COMMIT;",
    );

    if let Err(error) = db_setup_result {
        println!("{}", error);
        return Err(());
    }

    Ok(())
}

pub fn get_extensions(config: &Config) -> Vec<ExtensionProgression> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    // Get the extensions from the db
    let mut statement = connection
        .prepare("SELECT * FROM card INNER JOIN extension ON extension.id = card.extension_id")
        .unwrap();
    let res = statement.query_map([], |row| Ok(row.get::<&str, String>("card.id").unwrap()));
    // Convert the rows to a Vec<ExtensionProgression>
    for id in res.unwrap() {
        println!("{}", id.unwrap());
    }

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
