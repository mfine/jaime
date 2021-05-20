use reqwest::Error;
use serde::Deserialize;
use std::env;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

#[derive(Deserialize,Debug)]
struct Usd {
    rate: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
struct Bpi {
    USD: Usd,
}

#[derive(Deserialize,Debug)]
struct Btc {
    bpi: Bpi,
}

#[derive(Deserialize,Debug)]
struct Tweet {
    id_str: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
struct Item {
    bid: f64,
    ask: f64,
    marketCap: f64,
}

#[derive(Deserialize,Debug)]
struct Quotes {
    result: Vec<Item>,
}

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
struct Quote {
    quoteResponse: Quotes,
}

#[derive(Deserialize,Debug)]
struct Price {
    price: String,
}

#[derive(Deserialize,Debug)]
struct Data {
    prices: Vec<Price>,
}

#[derive(Deserialize,Debug)]
struct Prices {
    data: Data,
}

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
struct Comp {
    USD: f64,
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return the latest Bitcoin price.")]
    Btc,
    #[command(description = "return the latest Doge price.")]
    Doge,
    #[command(description = "return the latest Ethereum price.")]
    Eth,
    #[command(description = "return the latest Adrian Wojnarowski tweet.")]
    Woj,
    #[command(description = "return the latest wallstreetbets moderators tweet.")]
    Wsb,
    #[command(description = "return the latest tweet from a handle.")]
    Tweet(String),
    #[command(description = "return the latest Tesla quote.")]
    Tsla,
    #[command(description = "return the latest GameStop quote.")]
    Gme,
    #[command(description = "return the latest quote from a stock.")]
    Quote(String),
}

fn get_btc() -> Result<String, Error> {
    let response: Btc = reqwest::blocking::get("https://api.coindesk.com/v1/bpi/currentprice.json")?.json()?;
    Ok(response.bpi.USD.rate)
}

fn get_doge() -> Result<String, Error> {
    let client = reqwest::blocking::Client::new();
    let response: Prices = client
        .get("https://sochain.com/api/v2/get_price/DOGE/USD")
        .send()?.json()?;
    if response.data.prices.len() > 0 {
        Ok(response.data.prices[0].price.to_string())
    } else {
        Ok("https://bit.ly/2Yzl1GH".to_string())
    }
}

fn get_eth() -> Result<String, Error> {
    let client = reqwest::blocking::Client::new();
    let response: Comp = client
        .get("https://min-api.cryptocompare.com/data/price")
        .query(&[("fsym", "ETH"), ("tsyms", "USD")])
        .send()?.json()?;
    Ok(response.USD.to_string())
}

fn get_tweet(handle: String) -> Result<String, Error> {
    let client = reqwest::blocking::Client::new();
    let bearer = env::var("TWITTER_BEARER_TOKEN").unwrap();
    let response: Vec<Tweet> = client
        .get("https://api.twitter.com/1.1/statuses/user_timeline.json")
        .query(&[("count", "1"), ("screen_name", &handle)])
        .bearer_auth(bearer)
        .send()?.json()?;
    if response.len() > 0 {
        Ok(format!("https://twitter.com/{}/status/{}", handle, response[0].id_str))
    } else {
        Ok("https://bit.ly/2Yzl1GH".to_string())
    }
}

fn get_quote(stock: String) -> Result<String, Error> {
    let client = reqwest::blocking::Client::new();
    let key = env::var("RAPIDAPI_KEY").unwrap();
    let response: Quote = client
        .get("https://apidojo-yahoo-finance-v1.p.rapidapi.com/market/v2/get-quotes")
        .query(&[("region", "us"), ("symbols", &stock)])
        .header("x-rapidapi-key", key)
        .header("x-rapidapi-host", "apidojo-yahoo-finance-v1.p.rapidapi.com")
        .header("useQueryString", "true")
        .send()?.json()?;
    Ok(format!("{} {} {} {}", stock, response.quoteResponse.result[0].bid, response.quoteResponse.result[0].ask, response.quoteResponse.result[0].marketCap))
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?
        }
        Command::Btc => {
            match get_btc() {
                Ok(btc) => cx.answer(btc).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Doge => {
            match get_doge() {
                Ok(doge) => cx.answer(doge).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Eth => {
            match get_eth() {
                Ok(eth) => cx.answer(eth).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Woj => {
            match get_tweet("wojespn".to_string()) {
                Ok(woj) => cx.answer(woj).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Wsb => {
            match get_tweet("wsbmod".to_string()) {
                Ok(wsb) => cx.answer(wsb).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Tweet(handle) => {
            match get_tweet(handle) {
                Ok(tweet) => cx.answer(tweet).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Tsla => {
            match get_quote("TSLA".to_string()) {
                Ok(tsla) => cx.answer(tsla).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Gme => {
            match get_quote("GME".to_string()) {
                Ok(gme) => cx.answer(gme).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
        }
        Command::Quote(handle) => {
            match get_quote(handle) {
                Ok(quote) => cx.answer(quote).send().await?,
                _ => cx.answer("https://bit.ly/2Yzl1GH").send().await?,
            }
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
