#[derive(Debug)]
struct Match {
    home_name: String,
    home_goals: i8,
    away_name: String,
    away_goals: i8
}

fn main() {
    let url = "https://www.laliga.com/laliga-santander/resultados";
    let client = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let document = scraper::Html::parse_document(&client);
    let teams_selector = scraper::Selector::parse(".hvREvZ").unwrap();
    let mut teams = document.select(&teams_selector).map(|x| x.inner_html());
    let new_match: Match = Match {
        home_name: teams.nth(0).unwrap().to_string(),
        home_goals: 0,
        away_name: teams.nth(0).unwrap().to_string(),
        away_goals: 0
    };
    println!("{:#?}", new_match);
}
