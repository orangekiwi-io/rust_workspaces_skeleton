pub struct DataOne {
    pub value: i32,
}

pub fn process_one(data: &DataOne) -> i32 {
    data.value * 2
}

pub fn greet_one() -> String {
    "Hello from Workspace One!".to_string()
}
