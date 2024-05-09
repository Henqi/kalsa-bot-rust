use anyhow::Result;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

const API_KEY_NAME: &str = "TELOXIDE_TOKEN";
const API_URL: &str = "https://avoinna24.fi/api/slot";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Safari/605.1.15";

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<ShiftItem>,
}

#[derive(Debug, Deserialize)]
struct ShiftItem {
    // id: Option<i32>,
    // #[serde(rename = "type")]
    // data_type: String,
    attributes: Attributes,
    // relationships: Option<String>,
    // meta: Option<String>.
}

#[derive(Debug, Deserialize)]
struct Attributes {
    // "59305e30-8b49-11e9-800b-fa163e3c66dd"
    // #[serde(rename = "productId")]
    product_id: Option<String>,
    // "2024-05-09T06:30:00Z"
    starttime: Option<String>,
    //"2024-05-09T07:30:00Z"
    endtime: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let api_key: &str = &env::var(API_KEY_NAME).expect("TELOXIDE_TOKEN not found in .env");
    // println!("{}", api_key);

    let client = Client::builder().user_agent(USER_AGENT).build()?;
    check_hakis_availability(&client).await?;
    println!("-----");
    check_delsu_availability(&client).await?;
    Ok(())
}

async fn check_hakis_availability(client: &Client) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Subdomain", "arenacenter".parse()?);
    let date = "2024-05-09";

    let mut hakis: HashMap<&str, &str> = HashMap::new();
    hakis.insert("branch_id", "2b325906-5b7a-11e9-8370-fa163e3c66dd");
    hakis.insert("group_id", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd");
    hakis.insert("product_id", "59305e30-8b49-11e9-800b-fa163e3c66dd");
    hakis.insert("user_id", "d7c92d04-807b-11e9-b480-fa163e3c66dd");
    hakis.insert("date", date);
    hakis.insert("start", date);
    hakis.insert("end", date);

    let url = format!("https://avoinna24.fi/api/slot?filter[ismultibooking]=false&filter[branch_id]={}&filter[group_id]={}&filter[product_id]={}&filter[user_id]={}&filter[date]={}&filter[start]={}&filter[end]={}",
    hakis["branch_id"],
    hakis["group_id"],
    hakis["product_id"],
    hakis["user_id"],
    hakis["date"],
    hakis["start"],
    hakis["end"],
    );

    let response: ApiResponse = client
        .get(url.to_string())
        // .query()
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", response);
    Ok(())
}

async fn check_delsu_availability(client: &Client) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Subdomain", "arenacenter".parse()?);
    let date = "2024-05-09";

    let mut delsu: HashMap<&str, &str> = HashMap::new();
    delsu.insert("branch_id", "2b325906-5b7a-11e9-8370-fa163e3c66dd");
    delsu.insert("group_id", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd");
    delsu.insert("product_id", "59305e30-8b49-11e9-800b-fa163e3c66dd");
    delsu.insert("user_id", "ea8b1cf4-807b-11e9-93b7-fa163e3c66dd");
    delsu.insert("date", date);
    delsu.insert("start", date);
    delsu.insert("end", date);

    let url = format!("https://avoinna24.fi/api/slot?filter[ismultibooking]=false&filter[branch_id]={}&filter[group_id]={}&filter[product_id]={}&filter[user_id]={}&filter[date]={}&filter[start]={}&filter[end]={}",
    delsu["branch_id"],
    delsu["group_id"],
    delsu["product_id"],
    delsu["user_id"],
    delsu["date"],
    delsu["start"],
    delsu["end"],
    );

    let response: ApiResponse = client
        .get(url.to_string())
        // .query()
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", response);
    Ok(())
}
