use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut params = HashMap::<String, String>::new();
  params.insert("q".to_string(), "FC Barcelona".to_string());
  params.insert("location".to_string(), "austin, texas, united states".to_string());

  let search = SerpApiSearch::google(params, "secret_key".to_string());

  let results = search.json().await?;
  let sports_results = &results["sports_results"];
  println!("results received");
  println!("--- JSON ---");
  println!("{} - results:", sports_results);

  print!("ok");
  Ok(())
}
