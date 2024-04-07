mod get_cards;
mod get_number_of_cards;

#[derive(Debug, Error, Clone)]
pub enum ErrorKind {
    #[error("Could not get the number of cards")]
    NumberOfCardsError,
    #[error("Could not get the metadatas of the cards at the page: {page_number}")]
    GetMetadatasError { page_number: u32 },
    #[error("Could not fetch the informations of the card {card_number}")]
    DownloadCardError { card_number: String },
}

pub use get_cards::{download_card, get_cards, get_max_page};
pub use get_number_of_cards::get_number_of_cards;
use thiserror::Error;
