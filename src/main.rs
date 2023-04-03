fn main() {
    let url = "https://www.laliga.com/laliga-santander/resultados";
    let _class = "styled__ShieldContainer-lo8ov8-0 PIIgU";
    let client = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let document = scraper::Html::parse_document(&client);
    let teams_selector = scraper::Selector::parse(".hvREvZ").unwrap();
    let teams = document.select(&teams_selector).map(|x| x.inner_html());
    teams
        .zip(1..25)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
