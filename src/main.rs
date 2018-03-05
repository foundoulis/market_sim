
pub mod lib;

use lib::market::*;

fn main() {
    let num_agents = 101;
    let num_strats = 5;
    let history_len = 100;
    Market::new(num_agents, num_strats, history_len).tick_n(1600);
}
