// use scraper::{Html, Selector};

// pub async fn req_client(url: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // let mut laliga = reqwest::get(url);
//     // assert!(laliga.status().is_sucess());
//     // let body = Html::parse_document(&laliga.text().unwrap());
//     // let teams = Selector::parse(".styled__TextRegularStyled-sc-1raci4c-0 hvREvZ").unwrap();
//     // for teams in body.select(&teams) {
//     //     let teams = teams.text().collect::<Vec<_>>();
//     //     println!("{}", teams[0]);
//     // }
//     let client = reqwest::Client::new();
//     let laliga = client.get("https://www.laliga.com/laliga-santander/resultados")
//         .send()
//         .await?
//         .text()
//         .await?;
//     println!("{:#?}", laliga);

//     Ok(())
// }
pub async fn req_client() -> Result<(), reqwest::Error> {
    let _class = ".styled__TextRegularStyled-sc-1raci4c-0 hvREvZ";
    let laliga = reqwest::Client::new()
        .get("https://www.laliga.com/laliga-santander/resultados")
        .send()
        .await?
        .text()
        .await?;
    print!("{:#?}", laliga);
    Ok(())
}
