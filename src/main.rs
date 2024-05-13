use chrono::prelude::*;
use chrono::{DateTime, Duration, Timelike};
use chrono_tz::Europe::Helsinki;
use chrono_tz::Tz;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use std::env;

const API_KEY_NAME: &str = "TELOXIDE_TOKEN";
const API_URL: &str = "https://avoinna24.fi/api/slot";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Safari/605.1.15";
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

    let date: DateTime<Local> = Local::now();
    let next_day = date + Duration::days(1);
    let formatted_date = next_day.format("%Y-%m-%d").to_string();

    let mut hakis_parameters = vec![
        ("filter[ismultibooking]", "false"),
        ("filter[branch_id]", "2b325906-5b7a-11e9-8370-fa163e3c66dd"),
        ("filter[group_id]", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd"),
        ("filter[product_id]", "59305e30-8b49-11e9-800b-fa163e3c66dd"),
        ("filter[user_id]", "d7c92d04-807b-11e9-b480-fa163e3c66dd"),
        ("filter[date]", &formatted_date),
        ("filter[start]", &formatted_date),
        ("filter[end]", &formatted_date),
    ];

    let response: ApiResponse = client
        .get(API_URL.to_string())
        .query(&hakis_parameters)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

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
                continue;
            }
        }
    }
    Ok(())
}

async fn check_delsu_availability(client: &Client) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Subdomain", "arenacenter".parse()?);

    let date: DateTime<Local> = Local::now();
    let next_day = date + Duration::days(1);
    let formatted_date = next_day.format("%Y-%m-%d").to_string();

    let mut delsu_parameters = vec![
        ("filter[ismultibooking]", "false"),
        ("filter[branch_id]", "2b325906-5b7a-11e9-8370-fa163e3c66dd"),
        ("filter[group_id]", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd"),
        ("filter[product_id]", "59305e30-8b49-11e9-800b-fa163e3c66dd"),
        ("filter[user_id]", "ea8b1cf4-807b-11e9-93b7-fa163e3c66dd"),
        ("filter[date]", &formatted_date),
        ("filter[start]", &formatted_date),
        ("filter[end]", &formatted_date),
    ];

    let response: ApiResponse = client
        .get(API_URL.to_string())
        .query(&delsu_parameters)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

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
                continue;
            }
        }
    }
    Ok(())
}
