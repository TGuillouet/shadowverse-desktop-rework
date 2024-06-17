use crate::{cards::Card, config::Config};
use rusqlite::Connection;

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

    tracing::info!("{:?}", card);

    let result = connection.execute(
        "INSERT INTO 
            card (id, name, card_class, rarity, trait, type, details, extension_id, hp, cost, attack, is_evolved)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT DO UPDATE SET details = ?7",
        (
            &card.id,
            &card.name,
            &card.card_class,
            &card.rarity,
            &card.card_trait,
            &card.card_type,
            &card.details,
            &card.extension.id,
            &card.hp,
            &card.cost,
            &card.attack,
            &card.is_evolved
        ),
    );

    let Ok(_) = result else {
        tracing::error!("{:?}", result.err());
        return Err(());
    };

    // Add the card_collection
    let _ = connection.execute(
        "INSERT INTO
                collected_cards (card_id, is_owned)
            VALUES (?, ?)",
        (&card.id, false),
    );

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
