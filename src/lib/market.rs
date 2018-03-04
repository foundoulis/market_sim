
use lib::agent::Agent;

// Market
#[derive(Debug)]
struct Market {
    num_agents: u64,
    agents: Vec<Agent>,

    hist: MarketHistory,
}
impl Market {
    pub fn new(number_of_agents: u64) -> Market {
        
        Market {
            num_agents: number_of_agents,
            agents: Vec::with_capacity(number_of_agents as usize),
        }
    }
    pub fn tick_forward(&mut self) {

    }
    pub fn tick_forward_n(&mut self, n: u64) {
        for _ in 0..n {
            self.tick_forward();
        }
    }
}

// Market history
#[derive(Debug)]
struct MarketHistory {
    num_agents: u64
}
impl MarketHistory {
    pub fn new(number_of_agents: u64) -> MarketHistory {
        MarketHistory {
            num_agents: number_of_agents,
            
        }
    }
}
