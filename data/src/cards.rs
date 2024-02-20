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
