
pub mod lib;

use lib::market::*;
use std::thread;

fn main() {
    let num_agents = 5;
    let num_strats = 5;
    let history_len = 100;

    let mut values: Vec<std::thread::JoinHandle<Result<f64, ()>>> = Vec::new();

    for i in 0..8 {
        values.push(thread::Builder::new().name(format!("{}", i).to_string()).spawn(move || {
            let mut mark: Market = Market::new(num_agents, num_strats, history_len);
            mark.tick_n(1_000);
            Ok(mark.get_avg_price())
        }).unwrap());
    }
    for t in values {
        match t.join().unwrap() {
            Ok(s) => println!("Price: {}", s),
            Err(()) => {}
        }
    }    
}
