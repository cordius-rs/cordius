use serde_json::Value;

pub trait PayloadParser {
    fn get_id(&self) -> u64;
}

impl PayloadParser for Value {
    fn get_id(&self) -> u64 {
        self.as_object().unwrap().get("id").unwrap().as_str().unwrap().parse::<u64>().unwrap()
    }
}
