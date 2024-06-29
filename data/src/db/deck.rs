use rusqlite::Connection;

use crate::{
    config::Config,
    deck::{Deck, DeckCard},
};

pub fn create_table(config: &Config) {
    let connection =
        Connection::open(config.db_file.clone()).expect("Could open the database file");
}

pub fn create_deck(config: &Config) {}

pub fn add_card(deck: &Deck, deck_card: &DeckCard, config: &Config) {}

pub fn remove_card_from_deck(deck: &Deck, deck_card: &DeckCard, config: &Config) {}

pub fn update_card(deck: &Deck, deck_card: &DeckCard) {}
