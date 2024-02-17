pub struct Card {
    pub id: String,
    pub name: String,
    pub card_class: CardClass,
}

pub enum CardClass {
    Forestcraft,
    Swordcraft,
    Runecraft,
    Dragoncraft,
    Abysscraft,
    Havencraft,
}
