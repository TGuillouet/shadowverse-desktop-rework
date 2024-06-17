use crate::{
    cards::{Card, GameExtension},
    collection::CollectionCard,
    config::Config,
};
use rusqlite::Connection;

pub fn get_extension_cards(
    connection: &Connection,
    extension: &GameExtension,
) -> Vec<CollectionCard> {
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
            hp: row.get_unwrap("hp"),
            attack: row.get_unwrap("attack"),
            cost: row.get_unwrap("cost"),
            is_evolved: row.get_unwrap("is_evolved"),
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

pub fn update_card_quantity(config: &Config, card_id: &str, quantity: u8) -> Result<(), ()> {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");

    let _ = connection.execute(
        "UPDATE collected_cards SET quantity = ?1, is_owned = (CASE WHEN ?1 > 0 THEN 1 ELSE 0 END) WHERE card_id = ?2",
        (quantity, &card_id),
    );

    Ok(())
}
