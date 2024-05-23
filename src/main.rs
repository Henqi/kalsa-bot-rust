use anyhow::{Error, Result};
use chrono::prelude::*;
use chrono::{DateTime, Duration, Timelike};
use chrono_tz::Europe::Helsinki;
use chrono_tz::Tz;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};

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
    attributes: Attributes,
}

#[derive(Debug, Deserialize)]
struct Attributes {
    product_id: Option<String>,
    // "2024-05-09T06:30:00Z"
    starttime: Option<String>,
    //"2024-05-09T07:30:00Z"
    endtime: Option<String>,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Käynnistä botti!")]
    Start,
    #[command(description = "Onko kenttä vapaa?")]
    Help,
    #[command(description = "Onko Hakis vapaa?")]
    Hakis,
    #[command(description = "Onko Delsu vapaa?")]
    Delsu,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let _api_key: &str = &env::var(API_KEY_NAME).expect("TELOXIDE_TOKEN not found in .env");

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let client: Client = Client::builder().user_agent(USER_AGENT).build().unwrap();

    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Kalsa-bot, entistä ehompana! Powered by Rust™️")
                .await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Hakis => {
            let hakis_available = check_hakis_availability(&client).await;
            bot.send_message(msg.chat.id, hakis_available.unwrap())
                .await?
        }
        Command::Delsu => {
            let delsu_available = check_delsu_availability(&client).await;
            bot.send_message(msg.chat.id, delsu_available.unwrap())
                .await?
        }
    };

    Ok(())
}

async fn check_hakis_availability(client: &Client) -> Result<String, Error> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("X-Subdomain", "arenacenter".parse().unwrap());

    let next_day: DateTime<Local> = get_next_shift_date(Weekday::Wed);
    let formatted_date: String = next_day.format("%Y-%m-%d").to_string();

    let hakis_parameters: Vec<(&str, &str)> = vec![
        ("filter[ismultibooking]", "false"),
        ("filter[branch_id]", "2b325906-5b7a-11e9-8370-fa163e3c66dd"),
        ("filter[group_id]", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd"),
        ("filter[product_id]", "59305e30-8b49-11e9-800b-fa163e3c66dd"),
        ("filter[user_id]", "d7c92d04-807b-11e9-b480-fa163e3c66dd"),
        ("filter[date]", &formatted_date),
        ("filter[start]", &formatted_date),
        ("filter[end]", &formatted_date),
    ];

    let response = client
        .get(API_URL.to_string())
        .query(&hakis_parameters)
        .headers(headers)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(Error::msg(format!(
            "Request failed with status code: {}",
            response.status()
        )));
    }

    let json_response: ApiResponse = response.json().await?;

    if let Some(value) = get_free_shift_data(json_response, &HAKIS_SHIFT_ENDTIME, &formatted_date) {
        Ok(value)
    } else {
        Ok("EI DATAA!".to_string())
    }
}

async fn check_delsu_availability(client: &Client) -> Result<String, Error> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("X-Subdomain", "arenacenter".parse().unwrap());

    let next_day: DateTime<Local> = get_next_shift_date(Weekday::Tue);
    let formatted_date: String = next_day.format("%Y-%m-%d").to_string();

    let delsu_parameters: Vec<(&str, &str)> = vec![
        ("filter[ismultibooking]", "false"),
        ("filter[branch_id]", "2b325906-5b7a-11e9-8370-fa163e3c66dd"),
        ("filter[group_id]", "a17ccc08-838a-11e9-8fd9-fa163e3c66dd"),
        ("filter[product_id]", "59305e30-8b49-11e9-800b-fa163e3c66dd"),
        ("filter[user_id]", "ea8b1cf4-807b-11e9-93b7-fa163e3c66dd"),
        ("filter[date]", &formatted_date),
        ("filter[start]", &formatted_date),
        ("filter[end]", &formatted_date),
    ];

    let response = client
        .get(API_URL.to_string())
        .query(&delsu_parameters)
        .headers(headers)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(Error::msg(format!(
            "Request failed with status code: {}",
            response.status()
        )));
    }

    let json_response: ApiResponse = response.json().await?;

    if let Some(value) = get_free_shift_data(json_response, &DELSU_SHIFT_ENDTIME, &formatted_date) {
        Ok(value)
    } else {
        Ok("EI DATAA!".to_string())
    }
}

fn get_next_shift_date(weekday_target: Weekday) -> DateTime<Local> {
    let date_now: DateTime<Local> = Local::now();
    let weekday_now: Weekday = date_now.weekday();

    let days_until_next: i32 =
        (weekday_target.number_from_monday() as i32 - weekday_now.number_from_monday() as i32 + 7)
            % 7;

    date_now + Duration::days(days_until_next as i64)
}

fn get_free_shift_data(
    response: ApiResponse,
    shift_end_time: &u32,
    formatted_date: &str,
) -> Option<String> {
    for shift_item in response.data {
        if let Some(endtime) = &shift_item.attributes.endtime {
            let endtime: DateTime<Tz> = DateTime::parse_from_rfc3339(endtime)
                .unwrap()
                .with_timezone(&Helsinki);
            println!("Free shift endtimes: {}", endtime.to_rfc3339());
            if &endtime.hour() == shift_end_time {
                println!(
                    "Vuoro vapaana {}, joka loppuu tunnilla {}",
                    formatted_date,
                    endtime.hour()
                );
                return Some("Vapaa on!".to_string());
            } else {
                continue;
            }
        }
    }
    Some("EI VAPAATA VUOROA!".to_string())
}
