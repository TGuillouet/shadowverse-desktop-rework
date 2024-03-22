use std::{fmt::format, fs::File, io::BufWriter, path::PathBuf};

use scraper::selectable::Selectable;

use crate::get_number_of_cards::get_number_of_cards;

const CARDS_PER_PAGE: u32 = 15;
const PAGE_API_URL: &str = "https://en.shadowverse-evolve.com/cards/searchresults_ex?card_name=&class%5B0%5D=all&title=&expansion_name=&cost%5B0%5D=all&card_kind%5B0%5D=all&rare%5B0%5D=all&power_from=&power_to=&hp_from=&hp_to=&type=&ability=&keyword=&view=text&t=1711058152734&_=1711057240616";
const DETAIL_PAGE_URL: &str = "https://en.shadowverse-evolve.com/cards/?cardno=";

pub fn get_cards(covers_directory: &PathBuf) -> Vec<String> {
    let number_of_cards = get_number_of_cards().unwrap();

    download_card("BP03-001EN", covers_directory);

    get_cards_list(number_of_cards) // Get all the card numbers
}

pub fn download_card(card_number: &str, covers_directory: &PathBuf) {
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

    let card_number = html_card
        .select(&scraper::Selector::parse(".illustrator .name").unwrap())
        .next()
        .map(|p| p.text().collect::<String>())
        .unwrap();

    let infos_selector = scraper::Selector::parse(".info dl dd").unwrap();
    let mut infos = html_card.select(&infos_selector);

    let card_class = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_type = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_trait = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_rarity = infos.next().map(|p| p.text().collect::<String>()).unwrap();
    let card_extension = infos.next().map(|p| p.text().collect::<String>()).unwrap();

    let detail = html_card
        .select(&scraper::Selector::parse(".detail p").unwrap())
        .next()
        .map(|p| p.text().collect::<String>()) // TODO: find a way to get the images too
        .unwrap();

    let cover = get_image(&card_number, covers_directory);
    // let cover = html_card
    //     .select(&scraper::Selector::parse(".cardlist-Detail img").unwrap())
    //     .next()
    //     .map(|p| p.().collect::<String>()) // TODO: find a way to get the images too
    //     .unwrap();

    // TODO: Save the images in the application directory
    // TODO: Save the entries in the database
    println!(
        "{} - {} - {} - {} - {} - {} - {}",
        card_class, card_number, name, card_type, card_trait, card_rarity, card_extension
    );
}

fn get_cards_list(number_of_cards: u32) -> Vec<String> {
    let number_of_pages = number_of_cards.div_ceil(CARDS_PER_PAGE);
    let mut cards_number = Vec::new();

    for page_number in 1..=number_of_pages {
        let response = ureq::get(&format!("{}&page={}", PAGE_API_URL, page_number))
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
        println!("Page {}", page_number);
    }

    cards_number
}

fn get_image(card_number: &str, output_directory: &PathBuf) {
    let image_url = format!(
        "https://en.shadowverse-evolve.com/wordpress/wp-content/images/cardlist/BP03/{}.png",
        card_number
    );

    let mut response = ureq::get(&image_url).call().unwrap().into_reader();

    let mut out_file = File::create(output_directory.join(format!("{}.png", card_number))).unwrap();

    std::io::copy(&mut response, &mut BufWriter::new(&mut out_file));
}

// fn get_card_detail(card_number: &str) {
//for html_card in html_cards {
//     let name = html_card
//         .select(&scraper::Selector::parse(".ttl").unwrap())
//         .next()
//         .map(|p| p.text().collect::<String>())
//         .unwrap();
//
//     let card_number = html_card
//         .select(&scraper::Selector::parse(".number").unwrap())
//         .next()
//         .map(|p| p.text().collect::<String>())
//         .unwrap();
//
//     // TODO: Find a way to get the card class
//
//     let infos_selector = scraper::Selector::parse(".status span").unwrap();
//     let mut infos = html_card.select(&infos_selector);
//
//     let card_type = infos.next().map(|p| p.text().collect::<String>()).unwrap();
//     let card_trait = infos.next().map(|p| p.text().collect::<String>()).unwrap();
//     let card_rarity = infos.next().map(|p| p.text().collect::<String>()).unwrap();
//
//     let detail = html_card
//         .select(&scraper::Selector::parse(".detail").unwrap())
//         .next()
//         .map(|p| p.text().collect::<String>()) // TODO: find a way to get the images too
//         .unwrap();
//
//     println!(
//         "{} - {} - {} - {} - {}",
//         card_number, name, card_type, card_trait, card_rarity
//     );
// }

// }
