use std::fmt::Display;

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

#[derive(Debug, Clone)]
pub enum CardClass {
    Forestcraft,
    Swordcraft,
    Runecraft,
    Dragoncraft,
    Abysscraft,
    Havencraft,
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
            }
        )
    }
}
