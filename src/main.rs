
pub mod lib;

use lib::market::*;
use std::env;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_agents = args[1].parse::<u64>().unwrap_or(5);
    let num_strats = args[2].parse::<u64>().unwrap_or(3);
    let history_len = args[3].parse::<u64>().unwrap_or(10);
    let iterations = args[4].parse::<u64>().unwrap_or(1000);

    let mut values: Vec<std::thread::JoinHandle<Result<(f64, i32, i32), ()>>> = Vec::new();

    for i in 0..8 {
        values.push(thread::Builder::new().name(format!("{}", i).to_string()).spawn(move || {
            let mut mark: Market = Market::new(num_agents, num_strats, history_len);
            mark.tick_n(iterations);
            Ok((mark.get_avg_price(), mark.get_max_price(), mark.get_min_price()))
        }).unwrap());
    }
    println!("Prices: ");
    for t in values {
        match t.join().unwrap() {
            Ok(s) => println!("av: {}\tmax: {}\tmin: {}", s.0, s.1, s.2),
            Err(()) => {}
        }
    }
}
