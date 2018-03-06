
extern crate rand;

use lib::agent::Agent;
use self::rand::{thread_rng, Rng};

// Market
#[derive(Debug)]
pub struct Market {
    num_agents: u64,
    agents: Vec<Agent>,
    tick: u64,
    hist: MarketHistory,

    prices: Vec<i32>,
    sum: i64,
    average_price: f64,
}
impl Market {
    pub fn new(number_of_agents: u64, number_of_strategies: u64, history_length: u64) -> Market {
        let mut agentz = Vec::with_capacity(number_of_agents as usize);
        for _ in 0..number_of_agents {
            agentz.push(Agent::new(number_of_strategies));
        }
        Market {
            num_agents: number_of_agents,
            agents: agentz,
            tick: 0,
            hist: MarketHistory::new(number_of_agents, history_length),
            prices: Vec::new(),
            sum: 0,
            average_price: 0f64,
        }
    }
    pub fn tick(&mut self) {
        let mut new_history: Vec<bool> = Vec::with_capacity(self.num_agents as usize);
        let mut minority_position = 0;

        // As each of the agents to trade
        for agent in &mut self.agents {
            let agent_choice = agent.make_descision(self.hist.to_worker());
            if agent_choice {
                minority_position += 1;
            } else {
                minority_position -= 1;
            }
            new_history.push(agent_choice);
        }
        
        // Append new history
        self.hist.push_hist(&new_history);
        // Append to price history
        self.prices.push(minority_position);

        // Determine the minority position based upon market response.
        let minority = if minority_position < 0 {
            true
        } else {
            false
        };

        // Tell each agent how they did
        for agent in &mut self.agents {
            agent.update_history(self.hist.to_worker(), minority);
        }

        self.tick += 1;
        self.sum += minority_position as i64;
        self.average_price = (self.sum as f64)/(self.tick as f64);
    }
    pub fn tick_n(&mut self, n: u64) {
        for _ in 0..n {
            self.tick();
        }
        //println!("Prices for era {} to {}: {:?}", self.tick - n, self.tick, self.average_price);
    }
    pub fn get_avg_price(&self) -> f64{
        return self.average_price;
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
        let mut history: Vec<Vec<bool>> = Vec::new();
        for _ in 0..history_length {
            let mut tmp: Vec<bool> = Vec::new();
            for _ in 0..number_of_agents {
                tmp.push(thread_rng().gen());
            }
            history.push(tmp);
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
