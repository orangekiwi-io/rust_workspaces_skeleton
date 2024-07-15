use workspace_1::DataOne;

pub struct DataThree {
    pub base: DataOne,
    pub text: String,
}

pub fn process_three(data: &DataThree) -> String {
    format!("{}: {}", data.text, data.base.value)
}

pub fn greet_three() -> String {
    "3. Salutations from Workspace Three!".to_string()
}
