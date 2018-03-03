
use lib::strategy::Strategy;

// Agent
#[derive(Debug)]
struct Agent {
    strategies: Vec<Strategy>,
    history: Vec<u64>,
    num_strat: u64,
}
impl Agent {
    pub fn new(num_strat: u64) -> Agent {
        let mut strategies: Vec<Strategy> = Vec::new();
        let mut history: Vec<u64> = Vec::new();
        for _ in 0..num_strat {
            strategies.push(Strategy::new());
            history.push(0);
        }
        Agent {
            num_strat: num_strat,
            strategies: strategies,
            history: Vec::new(),
        }
    }
    pub fn make_descision(&self, hist: Vec<bool>) -> bool {
        
    }
}