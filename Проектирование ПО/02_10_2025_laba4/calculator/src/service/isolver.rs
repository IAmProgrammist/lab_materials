use std::error::Error;

pub trait ISolver {
    fn solve(&self, expression: &str) -> Result<String, Box<dyn Error>>; 
}