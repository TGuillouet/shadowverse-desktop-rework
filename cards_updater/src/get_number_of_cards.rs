const ALL_SHADOWVERSE_CARDS_URL: &str = "https://en.shadowverse-evolve.com/cards/searchresults/?card_name=&class%5B%5D=all&title=&expansion_name=&cost%5B%5D=all&card_kind%5B%5D=all&rare%5B%5D=all&power_from=&power_to=&hp_from=&hp_to=&type=&ability=&keyword=";

pub async fn get_number_of_cards() -> Result<u32, ureq::Error> {
    let response = ureq::get(ALL_SHADOWVERSE_CARDS_URL).call()?.into_string()?;

    let html = scraper::Html::parse_document(&response);
    let number: u32 = html
        .select(&scraper::Selector::parse(".cardlist-Result_Target_Num span.num").unwrap())
        .next()
        .map(|span| span.text().collect::<String>())
        .map(|nb_string| nb_string.parse().unwrap())
        .unwrap();

    Ok(number)
}
