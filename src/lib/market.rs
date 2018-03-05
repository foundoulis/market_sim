
extern crate rand;

use lib::agent::Agent;
use self::rand::{thread_rng, Rng};

// Market
#[derive(Debug)]
pub struct Market {
    num_agents: u64,
    agents: Vec<Agent>,

    hist: MarketHistory,
}
impl Market {
    pub fn new(number_of_agents: u64, number_of_strategies: u64, history_length: u64) -> Market {
        let mut agentz = Vec::with_capacity(number_of_agents as usize);
        for i in 0..number_of_agents {
            let i = i as usize;
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
        for agent in &mut self.agents {
            if agent.make_descision(self.hist.to_worker()) {
                minority_position += 1;
            } else {
                minority_position -= 1;
            }
        }

        // Determine the minority position based upon market response.
        let minority = if minority_position < 0 {
            false
        } else {
            true
        };

        // Tell each agent how they did
        for agent in &mut self.agents {
            agent.update_history(self.hist.to_worker(), minority);
        }
    }
    pub fn tick_forward_n_times(&mut self, n: u64) {
        for _ in 0..n {
            self.tick_forward();
        }
    }
}

// Market history
#[derive(Debug)]
struct MarketHistory {
    num_agents: u64, // Number of agents.
    hist_len: u64, // How far back each agent should be able to see when making a descision
    history: Vec<Vec<bool>>
}
impl MarketHistory {
    pub fn new(number_of_agents: u64, history_length: u64) -> MarketHistory {
        let mut history: Vec<Vec<bool>> = vec![vec![]];
        // To ensure the history is never empty.
        for i in 0..history_length {
            for _ in 0..number_of_agents {
                history[i as usize].push(thread_rng().gen());
            }
        }
        MarketHistory {
            num_agents: number_of_agents,
            hist_len: history_length,
            history: history,
        }
    }
    pub fn to_worker(&self) -> &Vec<bool> {
        // Pray to god the history is never negative.
        return &self.history.last().unwrap();
    }
    pub fn push_hist(&mut self, hist: &Vec<bool>) {
        self.history.push(hist.clone());
    }
}
