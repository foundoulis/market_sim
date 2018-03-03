
extern crate rand;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};

// Strategy
#[derive(Debug)]
pub struct Strategy {
    R: HashMap<Vec<bool>, bool>,
}
impl Strategy {
    pub fn new() -> Strategy {
        Strategy {
            R: HashMap::new(),
        }
    }
    pub fn get(&mut self, hist: &Vec<bool>) -> bool {
        if self.R.contains_key(hist) {
            return self.R.get(hist).unwrap().to_owned();
        } else {
            let new_value = thread_rng().gen();
            self.R.insert(hist.clone(), new_value);
            return new_value;
        }
    }
}
