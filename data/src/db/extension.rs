use rusqlite::Connection;

use crate::{cards::GameExtension, collection::ExtensionProgression, config::Config};

use super::collection::get_extension_cards;

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
