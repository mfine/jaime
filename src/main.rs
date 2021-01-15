use reqwest::Error;
use serde::Deserialize;
use std::env;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

#[derive(Deserialize)]
struct Usd {
    rate: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Bpi {
    USD: Usd,
}

#[derive(Deserialize)]
struct Btc {
    bpi: Bpi,
}

#[derive(Deserialize)]
struct Tweet {
    id_str: String,
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return the latest Bitcoin price.")]
    Btc,
    #[command(description = "return the latest Adrian Wojnarowski tweet.")]
    Woj,
    #[command(description = "return the latest tweet from a handle.")]
    Tweet(String),
}

fn get_btc() -> Result<String, Error> {
    let response: Btc = reqwest::blocking::get("https://api.coindesk.com/v1/bpi/currentprice.json")?.json()?;
    Ok(response.bpi.USD.rate)
}

fn get_tweet(handle: String) -> Result<String, Error> {
    let client = reqwest::blocking::Client::new();
    let bearer = env::var("TWITTER_BEARER_TOKEN").unwrap();
    let response: Vec<Tweet> = client
        .get("https://api.twitter.com/1.1/statuses/user_timeline.json")
        .query(&[("count", "1"), ("screen_name", &handle)])
        .bearer_auth(bearer)
        .send()?.json()?;
    Ok(format!("https://twitter.com/{}/status/{}", handle, response[0].id_str))
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?
        }
        Command::Btc => {
            let btc = get_btc().unwrap();
            cx.answer(btc).send().await?
        }
        Command::Woj => {
            let woj = get_tweet("wojespn".to_string()).unwrap();
            cx.answer(woj).send().await?
        }
        Command::Tweet(handle) => {
            let tweet = get_tweet(handle).unwrap();
            cx.answer(tweet).send().await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env();
    teloxide::commands_repl(bot, "jaime", answer).await;
}
