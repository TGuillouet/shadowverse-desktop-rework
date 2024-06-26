use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::ErrorKind;
use data::cards::{Card, CardClass, GameExtension};
use scraper::selectable::Selectable;

use crate::get_number_of_cards::get_number_of_cards;

const CARDS_PER_PAGE: u32 = 15;
const PAGE_API_URL: &str = "https://en.shadowverse-evolve.com/cards/searchresults_ex?card_name=&class%5B0%5D=all&title=&expansion_name=&cost%5B0%5D=all&card_kind%5B0%5D=all&rare%5B0%5D=all&power_from=&power_to=&hp_from=&hp_to=&type=&ability=&keyword=&view=text&t=1711058152734&_=1711057240616&sort=no";
const DETAIL_PAGE_URL: &str = "https://en.shadowverse-evolve.com/cards/?cardno=";

pub async fn get_max_page() -> u32 {
    let number_of_cards = get_number_of_cards().await.unwrap();
    number_of_cards.div_ceil(CARDS_PER_PAGE)
}

pub async fn get_cards(page_index: u32) -> Result<Vec<String>, ErrorKind> {
    let mut cards_number = Vec::new();

    let response = ureq::get(&format!("{}&page={}", PAGE_API_URL, page_index))
        .call()
        .unwrap()
        .into_string()
        .map_err(|_| ErrorKind::GetMetadatasError {
            page_number: page_index,
        })?;
    let html = scraper::Html::parse_document(&response);
    let cards_selector = scraper::Selector::parse("li").unwrap();
    let html_cards = html.select(&cards_selector);
    for html_card in html_cards {
        let card_number = html_card
            .select(&scraper::Selector::parse(".number").unwrap())
            .next()
            .map(|p| p.text().collect::<String>())
            .ok_or(ErrorKind::GetMetadatasError {
                page_number: page_index,
            })?;

        cards_number.push(card_number);
    }

    Ok(cards_number)
}

pub fn download_card(card_number: &str, covers_directory: &PathBuf) -> Result<Card, ErrorKind> {
    // Extract the data from the card detail page
    let response = ureq::get(&format!("{}{}", DETAIL_PAGE_URL, card_number))
        .call()
        .unwrap()
        .into_string()
        .map_err(|_| ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;
    let html_card = scraper::Html::parse_document(&response);

    let name = html_card
        .select(&scraper::Selector::parse(".ttl").unwrap())
        .next()
        .map(|p| p.text().collect::<String>())
        .ok_or(ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;

    let first_illustrator_selector = scraper::Selector::parse(".illustrator").unwrap();
    let card_number = html_card
        .select(&first_illustrator_selector)
        .next()
        .map(|span| {
            span.select(&scraper::Selector::parse("span").unwrap())
                .map(|span| span.text().collect::<String>())
                .last()
                .unwrap()
        })
        .ok_or(ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;

    let infos_selector = scraper::Selector::parse(".info dl").unwrap();
    let infos = html_card.select(&infos_selector);

    let card_class =
        get_from_block_with_text("Class", &infos).map_err(|_| ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;
    let card_type = get_from_block_with_text("Card Type", &infos).map_err(|_| {
        ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        }
    })?;

    let is_evolved = card_type.contains("Evolved");

    let card_trait =
        get_from_block_with_text("Trait", &infos).map_err(|_| ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;
    let card_rarity =
        get_from_block_with_text("Rarity", &infos).map_err(|_| ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;
    let card_extension =
        get_from_block_with_text("Card Set", &infos).map_err(|_| ErrorKind::DownloadCardError {
            card_number: card_number.to_string(),
        })?;

    let cost = html_card
        .select(&scraper::Selector::parse(".status span.status-Item-Cost").unwrap())
        .next()
        .map(|p| p.text().collect::<String>())
        .unwrap();

    let defense = html_card
        .select(&scraper::Selector::parse(".status span.status-Item-Hp").unwrap())
        .next()
        .map(|p| p.text().collect::<String>())
        .unwrap();

    let power = html_card
        .select(&scraper::Selector::parse(".status span.status-Item-Power").unwrap())
        .next()
        .map(|p| p.text().collect::<String>())
        .unwrap();

    let details = html_card
        .select(&scraper::Selector::parse(".detail p").unwrap())
        .next()
        .map(|p| p.html()) // TODO: find a way to get the images too
        .unwrap_or_default()
        .to_string();

    // get_image(&card_number, covers_directory);

    let extension_id = get_extension_id(&card_number);

    Ok(Card {
        id: card_number.to_string().clone(),
        name,
        card_class: CardClass::from(card_class),
        card_type,
        card_trait,
        rarity: card_rarity,
        hp: extract_number_from_str(&defense).parse::<u8>().unwrap_or(0),
        attack: extract_number_from_str(&power).parse::<u8>().unwrap_or(0),
        cost: extract_number_from_str(&cost).parse::<u8>().unwrap_or(0),
        is_evolved,
        details,
        extension: GameExtension {
            id: extension_id.to_string(),
            name: card_extension,
        },
    })
}

fn get_from_block_with_text(
    text_to_search: &str,
    infos: &scraper::html::Select,
) -> Result<String, ()> {
    let label_selector = scraper::Selector::parse("dt").unwrap();
    let content_selector = scraper::Selector::parse("dd").unwrap();
    for line in infos.clone() {
        let label = line
            .select(&label_selector)
            .next()
            .map(|dt| dt.text().collect::<String>())
            .ok_or(())?;
        let content = line
            .select(&content_selector)
            .next()
            .map(|dd| dd.text().collect::<String>())
            .ok_or(())?;

        if label == text_to_search {
            return Ok(content);
        }
    }

    Ok("Unknown".to_string())
}

fn get_extension_id(card_number: &str) -> &str {
    card_number.split('-').next().unwrap()
}

fn get_image(card_number: &str, output_directory: &Path) {
    let mut extension_id = get_extension_id(card_number);
    if extension_id.contains("BSF") {
        extension_id = "PR";
    }

    let image_url = format!(
        "https://en.shadowverse-evolve.com/wordpress/wp-content/images/cardlist/{}/{}.png",
        extension_id, card_number
    );

    let mut response = ureq::get(&image_url)
        .set("user-agent", "shadowverse-utils/0.1")
        .call()
        .unwrap()
        .into_reader();

    let mut out_file = File::create(output_directory.join(format!("{}.png", card_number))).unwrap();

    let _ = std::io::copy(&mut response, &mut BufWriter::new(&mut out_file));
}

fn extract_number_from_str(value: &str) -> &str {
    let numbers_regex = regex::Regex::from_str("[0-9]+").unwrap();

    let Some(numbers_match) = numbers_regex.find(value) else {
        return "";
    };

    numbers_match.as_str()
}
