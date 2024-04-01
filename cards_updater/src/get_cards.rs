use std::{fs::File, io::BufWriter, path::PathBuf};

use data::cards::{Card, CardClass, GameExtension};
use scraper::selectable::Selectable;

use crate::get_number_of_cards::get_number_of_cards;

// TODO: Create an error kind with thiserror and delete all the .unwrap()

const CARDS_PER_PAGE: u32 = 15;
const PAGE_API_URL: &str = "https://en.shadowverse-evolve.com/cards/searchresults_ex?card_name=&class%5B0%5D=all&title=&expansion_name=&cost%5B0%5D=all&card_kind%5B0%5D=all&rare%5B0%5D=all&power_from=&power_to=&hp_from=&hp_to=&type=&ability=&keyword=&view=text&t=1711058152734&_=1711057240616&sort=no";
const DETAIL_PAGE_URL: &str = "https://en.shadowverse-evolve.com/cards/?cardno=";

pub async fn get_cards(page: u32) -> Vec<String> {
    let cards_list = get_cards_list(page); // Get all the card numbers
    cards_list
}

pub async fn get_max_page() -> u32 {
    let number_of_cards = get_number_of_cards().await.unwrap();
    number_of_cards.div_ceil(CARDS_PER_PAGE)
}

pub fn download_card(card_number: &str, covers_directory: &PathBuf) -> Card {
    // Extract the data from the card detail page
    let response = ureq::get(&format!("{}{}", DETAIL_PAGE_URL, card_number))
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    let html_card = scraper::Html::parse_document(&response);

    let name = html_card
        .select(&scraper::Selector::parse(".ttl").unwrap())
        .next()
        .map(|p| p.text().collect::<String>())
        .unwrap();

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
        .unwrap();

    let infos_selector = scraper::Selector::parse(".info dl dd").unwrap();
    let mut infos = html_card.select(&infos_selector);

    let card_class = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_type = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_trait = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_rarity = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_extension = infos.next().map(|p| p.text().collect::<String>()).unwrap();

    let details = html_card
        .select(&scraper::Selector::parse(".detail p").unwrap())
        .next()
        .map(|p| p.html()) // TODO: find a way to get the images too
        .unwrap_or_default()
        .to_string();

    get_image(&card_number, covers_directory);

    let extension_id = get_extension_id(&card_number);

    Card {
        id: card_number.to_string().clone(),
        name,
        card_class: CardClass::from(card_class),
        card_type,
        card_trait,
        rarity: card_rarity,
        details,
        extension: GameExtension {
            id: extension_id.to_string(),
            name: card_extension,
        },
    }
}

fn get_extension_id(card_number: &str) -> &str {
    card_number.split("-").next().unwrap()
}

fn get_cards_list(page_index: u32) -> Vec<String> {
    let mut cards_number = Vec::new();

    let response = ureq::get(&format!("{}&page={}", PAGE_API_URL, page_index))
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    let html = scraper::Html::parse_document(&response);
    let cards_selector = scraper::Selector::parse("li").unwrap();
    let html_cards = html.select(&cards_selector);
    for html_card in html_cards {
        let card_number = html_card
            .select(&scraper::Selector::parse(".number").unwrap())
            .next()
            .map(|p| p.text().collect::<String>())
            .unwrap();

        cards_number.push(card_number);
    }
    println!("Page {}", page_index);

    cards_number
}

fn get_image(card_number: &str, output_directory: &PathBuf) {
    let mut extension_id = get_extension_id(card_number);
    if extension_id.contains("BSF") {
        extension_id = "PR";
    }

    let image_url = format!(
        "https://en.shadowverse-evolve.com/wordpress/wp-content/images/cardlist/{}/{}.png",
        extension_id, card_number
    );

    let mut response = ureq::get(&image_url).call().unwrap().into_reader();

    let mut out_file = File::create(output_directory.join(format!("{}.png", card_number))).unwrap();

    std::io::copy(&mut response, &mut BufWriter::new(&mut out_file));
}
