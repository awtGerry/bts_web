use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

struct Match {
    #[serde(rename="homeName")]
    home_name: String,
    #[serde(rename="homeGoals")]
    home_goals: i8,
    #[serde(rename="awayName")]
    away_goals: String,
    #[serde(rename="awayGoals")]
    away_res: i8
}

#[tokio::main]
pub async fn sp_search() -> Result<(), reqwest::Error> {
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
