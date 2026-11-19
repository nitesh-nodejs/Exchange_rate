use reqwest::Error;
use serde::Deserialize;
use std::{collections::HashMap, format, io::stdin, println};

// use crate::Exchanges::Success;
enum Commands {
    // Cnvt --INR 500 --USD
    Cnvt(HashMap<String, Option<u32>>, String),
    Cmpr(String, String, String),
    Exit(String),
    Error(String),
}
#[derive(Deserialize, Debug)]
struct ExchangeRates {
    date: String,
    base: String,
    quote: String,
    rate: f64,
}
#[derive(Debug)]
struct Urls(String);
#[derive(Debug)]
struct RowCommand(String);
#[tokio::main]
// Run these commands in your project folder to check against community standards:
// • cargo check: Fast check that code compiles cleanly without building full binaries.
// • cargo clippy: Lints your code and flags non-idiomatic patterns, unnecessary clones, or bad practices.
// • cargo fmt --check: Verifies formatting matches standard rustfmt style.
// • cargo test: Runs unit and integration tests.
async fn main() {
    println!("Cnvt --INR --100<Optional> --USD, Require--, Optional100");
    loop {
        let mut command: String = String::new();

        stdin()
            .read_line(&mut command)
            .expect("Couldn't read command");

        let command = RowCommand(command);
        let command: Commands = command_parser(command);

        match command {
            Commands::Exit(_exit) => {
                break;
            }
            Commands::Cnvt(from, to) => {
                for (key, _val) in from {
                    let main_url =
                        format!("https://api.frankfurter.dev/v2/rates?quotes={},{}", key, to);
                    let urls = Urls(main_url);
                    match fetch_currency(urls).await {
                        Ok(res) => {
                            compare_currency(res, &key, &to);
                        }
                        Err(error) => {
                            println!("FETCH ERROR-1: {}", error)
                        }
                    }
                }
            }
            Commands::Cmpr(_cmd, from, to) => {
                let main_url = format!(
                    "https://api.frankfurter.dev/v2/rates?quotes={},{}",
                    from, to
                );
                match fetch_currency(Urls(main_url)).await {
                    Ok(res) => {
                        compare_currency(res, &from, &to);
                    }
                    Err(error) => {
                        println!("FETCH ERROR-2: {}", error)
                    }
                }
            }
            Commands::Error(err) => println!("COMMAND MISMATCH: {err}"),
        }

        // let error = r#"{"err": "Invalid Currency Code", "code": 400}"#;
        // let success = r#"{"base": "Eur", "rate": }"#;

    }
}
async fn fetch_currency(main_url: Urls) -> Result<Vec<ExchangeRates>, Error> {
    // from ln 88 to 107 these code is equal to this operator "?"
    //   let response = reqwest::get(main_url.0).await?.json::<Vec<ExchangeRates>>().await?;
    //   Ok(response)
    // ------------------------VS------------------------------
    // let res = reqwest::get(main_url.0).await?;
    // if !res.status().is_success() {
    //     // let err = res.error_for_status().unwrap_err();
    //    return Err(res.error_for_status().unwrap_err());
    // };
    // let response = res.json::<Vec<ExchangeRates>>().await?;
    // Ok(response)
    // ------------------------VS------------------------------

    match reqwest::get(main_url.0).await {
        Ok(response) => {
            if !response.status().is_success() {
                println!("Server Error: {:?}", response.status());
                return Err(response.error_for_status().unwrap_err());
            }
            match response.json::<Vec<ExchangeRates>>().await {
                Ok(val) => Ok(val),
                Err(e) => Err(e),
            }
        }
        Err(e) => {
            if e.is_connect() || e.is_timeout() {
                println!("No internet connection or unreachable to server");
            } else {
                println!("Other network error: {:?}", e);
            }
            Err(e)
        }
    }
}

fn command_parser(commands: RowCommand) -> Commands {
    let exit = commands.0.trim().parse::<String>().expect("Couldn't parse");
    if exit == "exit" {
        return Commands::Exit(exit);
    }

    let command: Vec<&str> = commands.0.split("--").map(|cmd| cmd.trim()).collect();
    let comd: &str = command[0];
    let from: &str = command[1];
    //Compiler say: unnecessary closure used to substitute value for `Option::None`
    // let amt: Option<u32> = command.get(2).and_then(|_| None);
    let amt: Option<u32> = command.get(2).and(None);
    let to: &str = command[3];

    if comd == "Cnvt" {
        Commands::Cnvt(HashMap::from([(String::from(from), amt)]), String::from(to))
    } else if comd == "Cmpr" {
        Commands::Cmpr(String::from(comd), String::from(from), String::from(to))
    } else {
        Commands::Error(String::from("No such command exist"))
    }
}

fn compare_currency(response: Vec<ExchangeRates>, f: &str, t: &str) {
    println!("{} -> {}", f, t);
    if let Some(from) = response.iter().find(|rate| rate.quote == f)
        && let Some(to) = response.iter().find(|rate| rate.quote == t)
    {
        // if let Some(to) = response.iter().find(|rate| rate.quote == t) {
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
        // }
    };
}
