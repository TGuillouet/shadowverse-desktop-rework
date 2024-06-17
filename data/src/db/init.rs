use rusqlite::Connection;

use crate::config::Config;

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
            hp INTEGER NOT NULL,
            attack INTEGER NOT NULL,
            cost INTEGER NOT NULL,
            details TEXT NOT NULL,
            is_evolved INTEGER NOT NULL,
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
        tracing::error!("{:?}", error);
        return Err(());
    }

    let _ = connection.close();

    Ok(())
}
