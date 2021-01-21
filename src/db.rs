use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub short: String, // this is db primary key.
    pub long: String,

    // all other fields we can add later
}

impl Entry {
    pub fn new(short: String, long: String) -> Self {
        Self {
            short,
            long,
        }
    }
}

pub trait Db {
    fn add(&self, long: &str) -> Result<Entry>;
    fn get(&self, short: &str) -> Result<Entry>;
}

