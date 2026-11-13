use serde::Deserialize;
use std::{format, io::stdin, println};

#[derive(Deserialize, Debug)]
struct ExchangeRates {
    date: String,
    _base: String,
    quote: String,
    rate: f64,
}

#[tokio::main]
async fn main() {
    let mut command = String::new();
    let mut vectors: Vec<&str> = Vec::new();
    // let mut hash_map: HashMap<&str, &str> = HashMap::new();
    // let _map_hash = HashMap::from([("cmd", "convert")]);

    stdin()
        .read_line(&mut command)
        .expect("Couldn't read command");

    // cammand list cnvrt --USD --IND, cpmr --USD --IND, cmd --help

    let cmd: Vec<&str> = command.split("--").map(|cmd| cmd.trim()).collect();

    for cmds in cmd {
        vectors.push(cmds);
    }

    let _coms = vectors[0];
    let from = vectors[1];
    let to = vectors[2];

    println!("vecotrs: {:?},", vectors);
    // println!("cmd: {:?} from: {}, to: {}", coms, from, to);

    // ?base=USD&symbols=EUR,GBP
    let main_url = format!(
        "https://api.frankfurter.dev/v2/rates?quotes={},{}",
        from, to
    );
    // let url = format!("https://frankfurter.dev?base=USD&symbols={},{}", from, to);

    // println!("URL: {}", main_url);

    let response: Vec<ExchangeRates> = reqwest::get(main_url)
        .await
        .expect("reqwest failed")
        .json()
        .await
        .expect("Couldn't parse into json");

    // let cur: String = compare_currency(response);
    if let Some(currency) = compare_currency(response) {
        println!("{}", currency)
    }
    // for val in response {
    //     println!("Date: {:?}", val.date);
    //     println!("Base: {:?}", val.base);
    //     println!("Quote: {:?}", val.quote);
    //     println!("Rate: {:?}", val.rate);
    // }
}

// INR -> CNY
// EUR/CNY * EUR/INR
// EUR/CNY Rate * (EUR/INR) = 1 CNY = Rupee
//  7.4123 * 0.00959140 = 0.071CNY

// USD -> INR
//  EUR/INR : EUR/USD
// EUR/INR Rate * (EUR/USD) = 1 INR = USD

// r1 * (base2 / r2) =  q1

// 104.39 * (1 / 1.1139) = q1
// r1 * 0.89774665 = 93.71 INR

// Date: "2026-11-12"
// Base: "EUR"
// Quote: "JPY"
// Rate: 177.28
// Date: "2026-11-12"
// Base: "EUR"
// Quote: "RUB"
// Rate: 92.12
//  r1 * (1 / 92.12) = jpy
fn compare_currency(response: Vec<ExchangeRates>) -> Option<String> {
    // let command_list = vec!["cnvrt", "cmpr"];
    match response.as_slice() {
        [first, second, ..] => {
            let r1 = first.rate;
            let r2 = second.rate;

            // println!("First: {} -> {} at rate {}", first.base, first.quote, r1);
            // println!("Second: {} -> {} at rate {}", second.base, second.quote, r2);

            let exchange = format!(
                "As per date:{}, {}: {} -> {}: {}",
                first.date,
                first.quote,
                r1 * (1.0 / r2),
                second.quote,
                1
            );
            return Some(exchange);
        }
        _ => {
            println!(
                "Expected at least 2 exchange rates, but got {}",
                response.len()
            );
        }
    };
    None
}
