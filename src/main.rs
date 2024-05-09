use anyhow::Result;
use chrono::{DateTime, TimeZone, Timelike, Utc};
use chrono_tz::Europe::Helsinki;
use chrono_tz::Tz;
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
const DATE: &str = "2024-05-10";
const HAKIS_SHIFT_ENDTIME: u32 = 18;
const DELSU_SHIFT_ENDTIME: u32 = 19;

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
    let _api_key: &str = &env::var(API_KEY_NAME).expect("TELOXIDE_TOKEN not found in .env");
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

    let mut hakis: HashMap<&str, &str> = HashMap::new();
    hakis.insert("branch_id", "2b325906-5b7a-11e9-8370-fa163e3c66dd");
    hakis.insert("group_id", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd");
    hakis.insert("product_id", "59305e30-8b49-11e9-800b-fa163e3c66dd");
    hakis.insert("user_id", "d7c92d04-807b-11e9-b480-fa163e3c66dd");
    hakis.insert("date", DATE);
    hakis.insert("start", DATE);
    hakis.insert("end", DATE);

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

    for shift_item in response.data {
        if let Some(endtime) = &shift_item.attributes.endtime {
            let endtime: DateTime<Tz> = DateTime::parse_from_rfc3339(endtime)
                .unwrap()
                .with_timezone(&Helsinki);
            println!("Free shift endtimes: {}", endtime.to_rfc3339());
            if endtime.hour() == HAKIS_SHIFT_ENDTIME {
                println!(
                    "Vuoro vapaana, joka loppuu tunnilla {}",
                    endtime.hour().to_string()
                )
            } else {
                println!(
                    "TUNNILLA {} EI OLE LOPPUVIA VUOROJA",
                    endtime.hour().to_string()
                )
            }
        }
    }
    Ok(())
}

async fn check_delsu_availability(client: &Client) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Subdomain", "arenacenter".parse()?);

    let mut delsu: HashMap<&str, &str> = HashMap::new();
    delsu.insert("branch_id", "2b325906-5b7a-11e9-8370-fa163e3c66dd");
    delsu.insert("group_id", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd");
    delsu.insert("product_id", "59305e30-8b49-11e9-800b-fa163e3c66dd");
    delsu.insert("user_id", "ea8b1cf4-807b-11e9-93b7-fa163e3c66dd");
    delsu.insert("date", DATE);
    delsu.insert("start", DATE);
    delsu.insert("end", DATE);

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

    for shift_item in response.data {
        if let Some(endtime) = &shift_item.attributes.endtime {
            let endtime: DateTime<Tz> = DateTime::parse_from_rfc3339(endtime)
                .unwrap()
                .with_timezone(&Helsinki);
            println!("Free shift endtimes: {}", endtime.to_rfc3339());
            if endtime.hour() == DELSU_SHIFT_ENDTIME {
                println!(
                    "Vuoro vapaana, joka loppuu tunnilla {}",
                    endtime.hour().to_string()
                )
            } else {
                println!(
                    "TUNNILLA {} EI OLE LOPPUVIA VUOROJA",
                    endtime.hour().to_string()
                )
            }
        }
    }

    Ok(())
}
