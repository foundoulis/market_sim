
use lib::strategy::Strategy;

// Agent
#[derive(Debug)]
pub struct Agent {
    strategies: Vec<Strategy>, // A collection of strategies
    history: Vec<u64>, // A number of successes for each strategy.
    num_strat: u64,
}
impl Agent {
    pub fn new(num_strat: u64) -> Agent {
        let mut strategies: Vec<Strategy> = Vec::with_capacity(num_strat as usize);
        let mut history: Vec<u64> = Vec::with_capacity(num_strat as usize);
        for _ in 0..num_strat {
            strategies.push(Strategy::new());
            history.push(0);
        }
        Agent {
            num_strat: num_strat,
            strategies: strategies,
            history: history
        }
    }
    pub fn make_descision(&mut self, hist: &Vec<bool>) -> bool {
        let mut best_index: usize = 0; // index of the best strategy
        for i in 0..self.num_strat {
            if self.history[i as usize] > self.history[best_index] {
                best_index = i as usize;
            }
        }
        return self.strategies[best_index].get(hist);
    }
    pub fn update_history() {

    }
}