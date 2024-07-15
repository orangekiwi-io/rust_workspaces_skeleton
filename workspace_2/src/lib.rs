use workspace_1::DataOne;

pub struct DataTwo {
    pub base: DataOne,
    pub factor: f64,
}

pub fn process_two(data: &DataTwo) -> f64 {
    (data.base.value as f64) * data.factor
}

pub fn greet_two() -> String {
    "Greetings from Workspace Two!".to_string()
}
