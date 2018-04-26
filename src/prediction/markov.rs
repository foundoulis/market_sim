
use std::collections::BTreeMap;
use std::cmp::Ord;

struct WrongSizeKey;

#[derive(Debug)]
pub struct Markov<T: Ord + Copy> {
    step_size: u32, // Count of how far the markov chain will see back in time.
    map: BTreeMap<Vec<T>, BTreeMap<T, u32>>, // Maps prevalues to expected 
}
impl<T: Ord + Copy> Markov<T> {
    pub fn new(step_size: u32) -> Markov<T> {
        let the_map = BTreeMap::new();
        Markov {
            step_size: step_size,
            map: the_map,
        }
    }
    pub fn add(&mut self, values: &Vec<T>) -> Result<(),WrongSizeKey> {
        if values.len() < (self.step_size+1) as usize {
            return Err(WrongSizeKey);
        } else {
            for start in 0..(values.len()-self.step_size as usize) {
                let mut first_key: Vec<T> = Vec::new();
                for x in 0..self.step_size as usize {
                    first_key.push(values[start+x].clone());
                }
                let second_key: T = values[start+self.step_size as usize];
                if self.map.contains_key(&first_key) { // If we already have a value for the first key...
                    if self.map.get(&first_key).unwrap().contains_key(&second_key) { // And the second key has already been added
                        let value = self.map.get(&first_key).unwrap().get(&second_key).unwrap();
                        self.map.get(&first_key).unwrap().insert(&second_key, value + 1);
                    } else { // We have the first key, but not the second. 
                        self.map.get(&first_key).unwrap().insert(&second_key, 1);
                    }
                } else { // The map does not even have a first key.
                    let mut new_value: BTreeMap<T, u32> = BTreeMap::new();
                    new_value.insert(&second_key, 1);
                    self.map.insert(&first_key, new_value);
                }
            }
            return Ok(());
        }
    }
    pub fn get(&self, values: &Vec<T>) -> Result<T, WrongSizeKey> {
        if values.len() < self.step_size {
            return Err(WrongSizeKey);
        } else {
            let map = self.map.get(values);
            let mut total_weight = 0;
            let mut selected: T;
            for (key, value) in map {
                let weight = value;
                let r = thread_rng().gen::<u32>() % (total_weight + value);
                if r >= total_weight {
                    selected = key;
                }
                total_weight += value;
            }
            return Ok(selected);
        }
    }
}