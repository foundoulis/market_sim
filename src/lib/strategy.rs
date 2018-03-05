
extern crate rand;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};

// Strategy
#[derive(Debug)]
pub struct Strategy {
    r: HashMap<Vec<bool>, bool>,
    successes: u64,
}
impl Strategy {
    pub fn new() -> Strategy {
        Strategy {
            r: HashMap::new(),
            successes: 0
        }
    }
    pub fn get_bid(&mut self, hist: &Vec<bool>) -> bool {
        if self.r.contains_key(hist) {
            return self.r.get(hist).unwrap().to_owned();
        } else {
            let new_value = thread_rng().gen();
            self.r.insert(hist.clone(), new_value);
            return new_value;
        }
    }
    pub fn get_succ(&self) -> u64{
        return self.successes;
    }
    pub fn inc_succ(&mut self) {
        self.successes += 1;
    }
}
