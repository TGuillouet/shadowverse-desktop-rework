use std::fmt::Display;

use rusqlite::types::{FromSql, ValueRef};

#[derive(Debug, Clone)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub card_class: CardClass,
    pub extension: GameExtension,
}

#[derive(Debug, Clone)]
pub struct GameExtension {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardClass {
    Forestcraft,
    Swordcraft,
    Runecraft,
    Dragoncraft,
    Abysscraft,
    Havencraft,
    Neutral,
}

impl FromSql for CardClass {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        if let ValueRef::Text(value) = value {
            let value = String::from_utf8(value.to_vec()).unwrap_or_default();
            let class = match value.as_str() {
                "Forestcraft" => CardClass::Forestcraft,
                "Swordcraft" => CardClass::Swordcraft,
                "Dragoncraft" => CardClass::Dragoncraft,
                "Abysscraft" => CardClass::Abysscraft,
                "Havencraft" => CardClass::Havencraft,
                "Runecraft" => CardClass::Runecraft,
                _ => CardClass::Neutral,
            };
            return Ok(class);
        }

        Ok(CardClass::Neutral)
    }
}

impl CardClass {
    pub const ALL: [CardClass; 7] = [
        CardClass::Forestcraft,
        CardClass::Swordcraft,
        CardClass::Dragoncraft,
        CardClass::Abysscraft,
        CardClass::Havencraft,
        CardClass::Runecraft,
        CardClass::Neutral,
    ];
}

impl Display for CardClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardClass::Forestcraft => "Forestcraft",
                CardClass::Swordcraft => "Swordcraft",
                CardClass::Runecraft => "Runecraft",
                CardClass::Abysscraft => "Abysscraft",
                CardClass::Dragoncraft => "Dragoncraft",
                CardClass::Havencraft => "Havencraft",
                CardClass::Neutral => "Neutral",
            }
        )
    }
}
