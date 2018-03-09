
extern crate rustyline;

pub mod lib;

use lib::market::*;
use std::thread;

fn main() {
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
    let num_agents = 11;
    let num_strats = 10;
    let history_len = 100;
    let iterations = 10_000;

    let mut values: Vec<std::thread::JoinHandle<Result<(f64, i32, i32), ()>>> = Vec::new();

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
                    ))
                })
                .unwrap(),
        );
    }
    //println!("Prices: ");
    for t in values {
        match t.join().unwrap() {
            Ok(s) => println!("av: {:.3}\tmax: {}\tmin: {}", s.0, s.1, s.2),
            Err(()) => {
                println!("There was an error processing the output.", );
            }
        }
    }
}
