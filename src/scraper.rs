// use scraper::{Html, Selector};

struct Match {
    home_name: String,
    home_res: i8,
    away_name: String,
    away_res: i8
}

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let url = "https://www.laliga.com/laliga-santander/resultados";
    let laliga = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    print!("{:#?}", laliga);
    Ok(())
}
