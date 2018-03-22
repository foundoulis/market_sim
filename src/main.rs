
extern crate rustyline;

pub mod minority;

use minority::market::*;
use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn main() {
    let num_agents = 11;
    let num_strats = 10;
    let history_len = 100;
    let iterations = 10_000;

    let mut values: Vec<std::thread::JoinHandle<Result<(f64, i32, i32, Vec<i32>), ()>>> = Vec::new();

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
                        mark.get_prices(),
                    ))
                })
                .unwrap(),
        );
    }

    let mut all_prices: Vec<Vec<i32>>  = Vec::new();

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

    export(all_prices);
}

fn export(vector: Vec<Vec<i32>>) {
    let mut file = File::create("market_history.csv").unwrap();
    for inner_vec in vector {
        let mut line: String = String::from("");
        for price in inner_vec {
            line += format!("{}\t", price).as_str();
        }
        line += format!("\n").as_str();
        file.write_all(line.as_bytes());
    }
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
                        temp();
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

fn temp() {
}
