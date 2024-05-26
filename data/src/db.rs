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
            rarity VARCHAR(50) NOT NULL,
            trait VARCHAR(100) NOT NULL,
            type VARCHAR(50) NOT NULL,
            details TEXT NOT NULL,
            extension_id VARCHAR(50),
            FOREIGN KEY (extension_id) REFERENCES extension (id)
        );
        CREATE TABLE IF NOT EXISTS collected_cards (
            card_id VARCHAR(50),
            is_owned INTEGER,
            FOREIGN KEY (card_id) REFERENCES card (id)
        );
        CREATE UNIQUE INDEX IF NOT EXISTS collected_cards_card_id_IDX ON collected_cards (card_id);
        ALTER TABLE collected_cards ADD COLUMN quantity INTEGER DEFAULT 0;
        COMMIT;",
    );

    if let Err(error) = db_setup_result {
        println!("{}", error);
        return Err(());
    }

    let _ = connection.close();

    Ok(())
}

fn get_extension_cards(connection: &Connection, extension: &GameExtension) -> Vec<CollectionCard> {
    let mut statement = connection
        .prepare(
            "SELECT 
                *,
                cc.is_owned,
                cc.quantity,
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
            card_trait: row.get_unwrap("trait"),
            rarity: row.get_unwrap("rarity"),
            card_type: row.get_unwrap("type"),
            details: row.get_unwrap("details"),
        };
        let collection_card = CollectionCard {
            card,
            is_owned: row.get_unwrap("is_owned"),
            quantity: row.get_unwrap("quantity"),
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
    let mut statement = connection
        .prepare("SELECT * FROM extension ORDER BY name")
        .unwrap();
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

pub fn get_extension(config: &Config, extension_id: &str) -> ExtensionProgression {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    // Get the extension from the db
    let mut statement = connection
        .prepare("SELECT * FROM extension WHERE id = ?")
        .unwrap();
    let res = statement.query_row([extension_id], |row| {
        Ok(GameExtension {
            id: row.get_unwrap("id"),
            name: row.get_unwrap("name"),
        })
    });

    let extension = res.unwrap();
    let cards = get_extension_cards(&connection, &extension);

    ExtensionProgression {
        extension,
        extension_cards: cards,
    }
}

pub fn remove_card_from_collection(config: &Config, card: Card) -> Result<(), ()> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    let result = connection.execute(
        "INSERT INTO 
            collected_cards (card_id, is_owned)
        VALUES (?, ?)
        ON CONFLICT (card_id)
            DO UPDATE SET is_owned = excluded.is_owned, quantity = quantity - 1",
        (&card.id, false),
    );

    println!("{:?}", result);

    Ok(())
}

pub fn add_card_to_collection(config: &Config, card: Card) -> Result<(), ()> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    let result = connection.execute(
        "INSERT INTO 
            collected_cards (card_id, is_owned)
        VALUES (?, ?)
        ON CONFLICT (card_id)
            DO UPDATE SET is_owned = excluded.is_owned, quantity = excluded.quantity + 1",
        (&card.id, true),
    );

    println!("{:?}", result);

    Ok(())
}

pub fn upsert_card(config: &Config, card: Card) -> Result<(), ()> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    // Create the extension if needed
    let _ = connection.execute(
        "INSERT INTO 
            extension (id, name)
        VALUES (?, ?)",
        (&card.extension.id, &card.extension.name),
    );

    let result = connection.execute(
        "INSERT INTO 
            card (id, name, card_class, rarity, trait, type, details, extension_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT DO UPDATE SET details = ?7",
        (
            &card.id,
            &card.name,
            &CardClass::from(card.card_class),
            &card.rarity,
            &card.card_trait,
            &card.card_type,
            &card.details,
            &card.extension.id,
        ),
    );

    if let Ok(_) = result {
        // Add the card_collection
        let _ = connection.execute(
            "INSERT INTO
                collected_cards (card_id, is_owned)
            VALUES (?, ?)",
            (&card.id, false),
        );
    }

    Ok(())
}

pub fn get_all_cards_number(config: &Config) -> Vec<String> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    let mut statement = connection.prepare("SELECT id FROM card").unwrap();
    let cards_result = statement
        .query_map([], |row| Ok(row.get_unwrap("id")))
        .unwrap();

    let mut cards = Vec::new();
    for card in cards_result {
        cards.push(card.unwrap());
    }
    cards
}

pub fn update_card_quantity(config: &Config, card_id: &str, quantity: u8) -> Result<(), ()> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    let _ = connection.execute(
        "UPDATE collected_cards SET quantity = ?1, is_owned = (CASE WHEN ?1 > 0 THEN 1 ELSE 0 END) WHERE card_id = ?2",
        (quantity, &card_id),
    );

    Ok(())
}
