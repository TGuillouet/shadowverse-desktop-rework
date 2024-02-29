use crate::{
    cards::{Card, GameExtension},
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

fn get_extension_cards(connection: &Connection, extension: &GameExtension) -> Vec<CollectionCard> {
    let mut statement = connection
        .prepare(
            "SELECT 
                *,
                cc.is_owned,
                e.id as extension_id,
                e.name as extension_name
            FROM card
            INNER JOIN extension e ON e.id = card.extension_id
            INNER JOIN collected_cards cc ON cc.card_id = card.id
            WHERE e.id = ?",
        )
        .unwrap();
    let res = statement.query_map([&extension.id], |row| {
        let extension = GameExtension {
            id: row.get_unwrap("extension_id"),
            name: row.get_unwrap("extension_name"),
        };
        let card = Card {
            id: row.get_unwrap("id"),
            extension,
            card_class: row.get_unwrap("card_class"),
            name: row.get_unwrap("name"),
        };
        let collection_card = CollectionCard {
            card,
            is_owned: row.get_unwrap("is_owned"),
        };
        Ok(collection_card)
    });

    let mut collected_cards = Vec::new();
    for card in res.unwrap() {
        collected_cards.push(card.unwrap());
    }
    collected_cards
}

pub fn get_extensions(config: &Config) -> Vec<ExtensionProgression> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    // Get the extensions from the db
    let mut statement = connection.prepare("SELECT * FROM extension").unwrap();
    let res = statement.query_map([], |row| {
        Ok(GameExtension {
            id: row.get_unwrap("id"),
            name: row.get_unwrap("name"),
        })
    });

    // Convert the rows to a Vec<ExtensionProgression>
    let mut extensions: Vec<ExtensionProgression> = Vec::new();
    for extension in res.unwrap() {
        let extension = extension.unwrap();
        let cards = get_extension_cards(&connection, &extension);
        extensions.push(ExtensionProgression {
            extension,
            extension_cards: cards,
        })
    }

    extensions
}
