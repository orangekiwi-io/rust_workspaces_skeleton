extern crate crate_workspaces;
use crate_workspaces::api::{run_all_greetings, DataOne, DataThree, DataTwo};

fn main() {
    println!("*** root_example in examples ***\n");
    // #[cfg(feature = "workspace_1")]
    // {
    let data_one = DataOne::new(5);
    println!("DataOne processing result: {}", data_one.process());
    // }

    // #[cfg(feature = "workspace_2")]
    // {
    let data_two = DataTwo::new(DataOne::new(10), 1.5);
    println!("DataTwo processing result: {}", data_two.process());
    // }

    // #[cfg(feature = "workspace_3")]
    // {
    let data_three = DataThree::new(DataOne::new(7), "Lucky number".to_string());
    println!("DataThree processing result: {}", data_three.process());
    // }

    // Running all greetings
    println!("\nAll greetings:");
    for greeting in run_all_greetings() {
        println!("- {}", greeting);
    }
    println!();

    // Demonstrating feature-gated functionality
    // #[cfg(not(feature = "workspace_1"))]
    // println!("Workspace 1 is not enabled");

    // #[cfg(not(feature = "workspace_2"))]
    // println!("Workspace 2 is not enabled");

    // #[cfg(not(feature = "workspace_3"))]
    // println!("Workspace 3 is not enabled");
}
