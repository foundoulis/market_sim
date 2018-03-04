
use lib::agent::Agent;

// Market
#[derive(Debug)]
struct Market {
    num_agents: u64,
    agents: Vec<Agent>,

    hist: MarketHistory,
}
impl Market {
    pub fn new(number_of_agents: u64, number_of_strategies: u64, history_length: u64) -> Market {
        let mut agentz = Vec::with_capacity(number_of_agents as usize);
        for i in number_of_agents {
            agentz[i] = Agent::new(number_of_strategies);
        }
        Market {
            num_agents: number_of_agents,
            agents: agentz,

            hist: MarketHistory::new(number_of_agents, history_length),
        }
    }
    pub fn tick_forward(&mut self) {
        let mut minority_position = 0;

        // As each of the agents to trade
        for agent in self.agents {
            if agent.make_descision(self.hist) {
                minority_position += 1;
            } else {
                minority_position -= 1;
            }
        }

        // Determine the minority position


        // Tell each agent how they did
        for agent in self.agents {
            agent.update_history(self.hist);
        }
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
    num_agents: u64,
    hist_len: u64, // How far back each agent should be able to see when making a descision
}
impl MarketHistory {
    pub fn new(number_of_agents: u64, history_length: u64) -> MarketHistory {
        MarketHistory {
            num_agents: number_of_agents,
            hist_len: history_length,
        }
    }
}
