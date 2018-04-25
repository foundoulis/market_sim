
use std::collections::BTreeMap;
use std::cmp::Ord;


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
    pub fn add(&mut self, values: &Vec<T>) -> Result<(),()> {
        if values.len() < (self.step_size+1) as usize {
            return Err(());
        } else {
            for start in 0..(values.len()-self.step_size as usize) {
                let mut first_key: Vec<T> = Vec::new();
                for x in 0..self.step_size as usize {
                    first_key.push(values[start+x].clone());
                }
                let second_key: T = values[start+self.step_size as usize];
                if self.map.contains_key(&first_key) {
                    if self.map.get(&first_key).unwrap().contains_key(&second_key) {
                        self.map.get(&first_key).unwrap().insert(second_key, );
                    }
                } else {

                }
            }
            return Ok(());
        }
    }
}