use serde::Deserialize;
use std::{collections::HashMap, format, io::stdin, println};

#[derive(Deserialize, Debug)]
struct ExchangeRates {
    date: String,
    base: String,
    quote: String,
    rate: f64,
}
// {
//     amount: f64,
//     rates: HashMap<String, f64>,
// }
#[tokio::main]
async fn main() {
    let mut command = String::new();
    let mut vectors: Vec<&str> = Vec::new();
    // let mut hash_map: HashMap<&str, &str> = HashMap::new();

    let _map_hash = HashMap::from([("cmd", "convert")]);

    stdin()
        .read_line(&mut command)
        .expect("Couldn't read command");

    // cammand list cnvrt --USD --IND, cpmr --USD --IND, cmd --help

    let cmd: Vec<&str> = command.split("--").map(|cmd| cmd.trim()).collect();

    for cmds in cmd {
        // println!("cmds: {:?},", cmds);
        vectors.push(cmds);
    }

    let coms = vectors[0];
    let from = vectors[1];
    let to = vectors[2];

    println!("vecotrs: {:?},", vectors);
    println!("cmd: {:?} from: {}, to: {}", coms, from, to);

    // ?base=USD&symbols=EUR,GBP
    let main_url = format!(
        "https://api.frankfurter.dev/v2/rates?quotes={},{}",
        from, to
    );
    // let url = format!("https://frankfurter.dev?base=USD&symbols={},{}", from, to);

    println!("URL: {}", main_url);

    let response: Vec<ExchangeRates> = reqwest::get(main_url)
        .await
        .expect("reqwest failed")
        .json()
        .await
        .expect("Couldn't parse into json");

    println!("response: {:?}", response);
}
