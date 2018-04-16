#![allow(unused_lints)]

extern crate rustyline;

pub mod minority;

use minority::market::*;
use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn main() {
    let num_agents = 100;
    let num_strats = 3;
    let history_len = 100;
    let iterations = 1_000;

    let mut values: Vec<std::thread::JoinHandle<Result<(f64, i32, i32, Vec<i32>, Vec<i32>), ()>>> = Vec::new();

    for i in 0..8 {
        values.push(
            thread::Builder::new()
                .name(format!("{}", i).to_string())
                .spawn(move || {
                    let mut mark: Market = Market::new(num_agents, num_strats, history_len);
                    mark.tick_n(iterations);
                    Ok((
                        mark.get_avg_price(),
                        mark.get_max_price(),
                        mark.get_min_price(),
                        mark.get_price_changes(),
                        mark.get_price_history()
                    ))
                })
                .unwrap(),
        );
    }

    let mut all_prices: Vec<Vec<i32>> = Vec::new();

    for t in values {
        match t.join().unwrap() {
            Ok(s) => {
                //println!("av: {:.3}\tmax: {}\tmin: {}", s.0, s.1, s.2);
                all_prices.push(s.3);
            },
            Err(()) => {
                println!("There was an error processing the output.", );
            }
        }
    }    

    export(&all_prices);
}

fn export(vector: &Vec<Vec<i32>>) {
    let mut file = File::create("market_history.csv").unwrap();
    for (x, inner_vec) in vector.iter().enumerate() { // TODO: Change this loop to a map for simplicity.
        let mut curr_price: i64 = 0;
        let mut line: String = String::from("");
        for price in inner_vec {
            curr_price += *price as i64;
            line += format!("{}\t", curr_price).as_str();
        }
        line += format!("\n").as_str();
        file.write_all(line.as_bytes()).unwrap_or_else(|_| println!("Failed export at line {}.", x));
    }
    // Create an average line.
    let mut line: String = String::from("");
    let length_of_history = vector[0].len();
    let number_of_markets = vector.len();
    let mut instant_price: Vec<i64> = Vec::new();
    for _ in 0..number_of_markets {
        instant_price.push(0);
    }
    for x in 0..length_of_history {
        for i in 0..number_of_markets {
            instant_price[i] = instant_price[i] + vector[i][x] as i64;
        }
        let mut sum: i64 = 0;
        for i_price in &instant_price {
            sum += i_price;
        }
        line += format!("{}\t", sum/number_of_markets as i64).as_str();
    }
    file.write_all(line.as_bytes()).unwrap_or_else(|_| println!("Failed to export final market average line."));
}

fn give_command_line() -> () {
    let mut rl = rustyline::Editor::<()>::new();
    if let Err(_) = rl.load_history("market-sim.txt") {
        println!("No previous history found.");
    }
    parse_line(&mut rl);
    rl.save_history("market-sim.txt").unwrap();
}

fn parse_line(rl: &mut rustyline::Editor<()>) {
    loop {
        let readline = rl.readline("|)]}> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "quit" {
                    break
                } else if line == "run" {
                    thread::spawn(move || {
                        //temp();
                    });
                } else {
                    println!("{}", line);
                }
            },
            Err(_) => {
                println!("No input, \"quit\" to exit");
            },
        }
    }
}
