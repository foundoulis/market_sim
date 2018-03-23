
extern crate rand;

use minority::strategy::Strategy;
use self::rand::{thread_rng, Rng};

// Agent
#[derive(Debug)]
pub struct Agent {
    strategies: Vec<Strategy>, // A collection of strategies
    num_strat: u64,
    acts_irrationally: f64,
}
impl Agent {
    pub fn new(num_strat: u64, irrational: f64) -> Agent {
        let mut strategies: Vec<Strategy> = Vec::with_capacity(num_strat as usize);
        for _ in 0..num_strat {
            strategies.push(Strategy::new());
        }
        Agent {
            num_strat: num_strat,
            strategies: strategies,
            acts_irrationally: irrational,
        }
    }
    pub fn make_descision(&mut self, hist: &Vec<bool>) -> bool {
        if self.acts_irrationally < thread_rng().gen_range::<f64>(0f64, 100f64) {
            return thread_rng().gen();
        }
        let mut best_index: usize = 0; // index of the best strategy
        for i in 0..self.num_strat {
            let i = i as usize;
            if self.strategies[i].get_succ() > self.strategies[best_index].get_succ() {
                best_index = i;
            }
        }
        return self.strategies[best_index].get_bid(hist);
    }
    pub fn update_history(&mut self, hist: &Vec<bool>, minority_position: bool) {
        // Update whether or not a given strategy would have worked
        for strat in &mut self.strategies {
            if minority_position == strat.get_bid(hist) {
                strat.inc_succ();
            }
        }
    }
}
