use reqwest::Error;
use serde::Deserialize;
use std::{collections::HashMap, format, io::stdin, println};

// use crate::Exchanges::Success;
enum Commands {
    // CNVT --INR 500 --USD
    CNVT(HashMap<String, Option<u32>>, String),
    CMPR(String, String, String),
    Error(String),
}
#[derive(Deserialize, Debug)]
struct ExchangeRates {
    date: String,
    base: String,
    quote: String,
    rate: f64,
}

#[tokio::main]
async fn main() {
    println!("CNVT --INR --100:Optional --USD, Require--, Optional100");
    loop {
        let mut command = String::new();

        stdin()
            .read_line(&mut command)
            .expect("Couldn't read command");

        let command = command_parser(command);

        match command {
            Commands::CNVT(from, to) => {
                for (key, _val) in from {
                    let main_url =
                        format!("https://api.frankfurter.dev/v2/rates?quotes={},{}", key, to);
                    match fetch_currency(main_url).await {
                        Ok(res) => {
                            compare_currency(res, &key, &to);
                        }
                        Err(error) => {
                            println!("{}", error)
                        }
                    }
                }
            }
            Commands::CMPR(_cmd, from, to) => {
                let main_url = format!(
                    "https://api.frankfurter.dev/v2/rates?quotes={},{}",
                    from, to
                );
                match fetch_currency(main_url).await {
                    Ok(res) => {
                        compare_currency(res, &from, &to);
                    }
                    Err(error) => {
                        println!("{}", error)
                    }
                }
            }
            Commands::Error(err) => println!("{err}"),
        }

        // let url = format!("https://frankfurter.dev?base=USD&symbols={},{}", from, to);

        // let error = r#"{"err": "Invalid Currency Code", "code": 400}"#;
        // let success = r#"{"base": "Eur", "rate": }"#;

        // [
        // ExchangeRates { date: "2026-11-14", base: "EUR", quote: "CNY", rate: 7.3874 },
        // ExchangeRates { date: "2026-11-13", base: "EUR", quote: "INR", rate: 104.22 }
        // ]
    }
}
async fn fetch_currency(main_url: String) -> Result<Vec<ExchangeRates>, Error> {
    match reqwest::get(main_url).await {
        Ok(response) => match response.json::<Vec<ExchangeRates>>().await {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        },
        Err(e) => {
            return Err(e);
        }
    }
}

fn command_parser(commands: String) -> Commands {
    let command: Vec<&str> = commands.split("--").map(|cmd| cmd.trim()).collect();

    let comd = command[0];
    let from = command[1];
    let amt = command.get(2).and_then(|_| None);
    let to = command[3];

    if comd == "CNVT" {
        Commands::CNVT(HashMap::from([(String::from(from), amt)]), String::from(to))
    } else if comd == "CMPR" {
        Commands::CMPR(String::from(comd), String::from(from), String::from(to))
    } else {
        Commands::Error(String::from("No such command exist"))
    }
}

fn compare_currency(response: Vec<ExchangeRates>, f: &str, t: &str) {
    println!("f: {}, t: {}", f, t);
    if let Some(from) = response.iter().find(|rate| rate.quote == f) {
        if let Some(to) = response.iter().find(|rate| rate.quote == t) {
            let exchange = format!(
                "As per date:{}, of base: {}, {}: {} -> {}: {}",
                from.date,
                from.base,
                from.quote,
                from.rate * (1.0 / to.rate),
                1,
                to.quote,
            );
            println!("{:?}", exchange);
        }
    };
}
