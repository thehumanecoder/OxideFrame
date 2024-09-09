use serde::{Deserialize, Serialize};

// A struct representing a record for serialization/deserialization.
// This is a simplified structure where each record has a name and an age.
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub name: String,
    pub age: i32,
}

// Function to load data from a JSON string and deserialize it into a vector of `Record`.
pub fn load_from_json(data: &str) -> Vec<Record> {
    serde_json::from_str(data).unwrap() // Deserialize JSON string into vector of `Record`
}

// Function to serialize a vector of `Record` into a JSON string.
pub fn save_to_json(records: &[Record]) -> String {
    serde_json::to_string(records).unwrap() // Serialize vector of `Record` to JSON string
}
