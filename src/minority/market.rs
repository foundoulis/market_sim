
extern crate rand;

use minority::agent::Agent;
use self::rand::{thread_rng, Rng};

// Market
#[derive(Debug)]
pub struct Market {
    num_agents: u64,
    agents: Vec<Agent>,
    tick: u64,
    hist: MarketHistory,
    price_history: Vec<i32>,
    price_change: Vec<i32>,
    sum: i64,
    average_price: f64,
    max_price: i32,
    min_price: i32,
}
impl Market {
    pub fn new(number_of_agents: u64, number_of_strategies: u64, history_length: u64) -> Market {
        let mut agentz = Vec::with_capacity(number_of_agents as usize);
        for _ in 0..number_of_agents {
            agentz.push(Agent::new(number_of_strategies, 0.0f64));
        }
        Market {
            num_agents: number_of_agents,
            agents: agentz,
            tick: 0,
            hist: MarketHistory::new(number_of_agents, history_length),
            price_history: Vec::new(),
            price_change: Vec::new(),
            sum: 0,
            average_price: 0f64,
            min_price: i32::max_value(),
            max_price: i32::min_value(),
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
        self.price_change.push(minority_position);
        let last_price = self.price_history.last().unwrap_or(&0).clone();
        self.price_history.push(minority_position + last_price);

        // Determine the minority position based upon market response.
        let minority = if minority_position < 0 { true } else { false };

        // Tell each agent how they did
        for agent in &mut self.agents {
            agent.update_history(self.hist.to_worker(), minority);
        }

        // Data tracking stuff.
        self.tick += 1;
        self.sum += minority_position as i64;
        self.average_price = (self.sum as f64) / (self.tick as f64);
        if minority_position > self.max_price {
            self.max_price = minority_position;
        } else if minority_position < self.min_price {
            self.min_price = minority_position;
        }
    }
    pub fn tick_n(&mut self, n: u64) {
        for _ in 0..n {
            self.tick();
        }
        //println!("price_change for era {} to {}: {:?}", self.tick - n, self.tick, self.average_price);
    }
    pub fn get_price_changes(&self) -> Vec<i32> {
        return self.price_change.clone();
    }
    pub fn get_price_history(&self) -> Vec<i32> {
        return self.price_history.clone();
    }
    pub fn get_avg_price(&self) -> f64 {
        return self.average_price;
    }
    pub fn get_min_price(&self) -> i32 {
        return self.min_price;
    }
    pub fn get_max_price(&self) -> i32 {
        return self.max_price;
    }
}

// Market history
#[derive(Debug)]
struct MarketHistory {
    num_agents: u64, // Number of agents.
    hist_len: u64,   // How far back each agent should be able to see when making a descision
    history: Vec<Vec<bool>>,
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
