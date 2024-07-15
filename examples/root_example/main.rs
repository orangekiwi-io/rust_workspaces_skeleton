extern crate crate_workspaces;
#[cfg(feature = "full")]
use crate_workspaces::api::run_all_greetings;

#[cfg(feature = "feature_1")]
use crate_workspaces::api::DataOne;

#[cfg(feature = "feature_2")]
use crate_workspaces::api::DataTwo;

#[cfg(feature = "feature_3")]
use crate_workspaces::api::DataThree;

fn main() {
    println!("*** Examples: root_example ***\n");
    #[cfg(feature = "feature_1")]
    {
        let data_one = DataOne::new(5);
        println!("DataOne processing result: {}", data_one.process());
    }

    #[cfg(feature = "feature_2")]
    {
        let data_two = DataTwo::new(DataOne::new(10), 1.5);
        println!("DataTwo processing result: {}", data_two.process());
    }

    #[cfg(feature = "feature_3")]
    {
        let data_three = DataThree::new(DataOne::new(7), "Lucky number".to_string());
        println!("DataThree processing result: {}", data_three.process());
    }

    // Running all greetings
    #[cfg(feature = "full")]
    {
        println!("\nAll greetings:");
        for greeting in run_all_greetings() {
            println!("- {}", greeting);
        }
        println!();
    }

    // Demonstrating feature-gated functionality
    #[cfg(any(feature = "feature_1", feature = "feature_2", feature = "feature_3"))]
    println!("==========================");
    #[cfg(feature = "feature_1")]
    println!("Feature 1 HAS been enabled");
    #[cfg(not(feature = "feature_1"))]
    println!("Workspace 1 is not enabled");

    #[cfg(feature = "feature_2")]
    println!("Feature 2 HAS been enabled");
    #[cfg(not(feature = "feature_2"))]
    println!("Workspace 2 is not enabled");

    #[cfg(feature = "feature_3")]
    println!("Feature 3 HAS been enabled");
    #[cfg(not(feature = "feature_3"))]
    println!("Workspace 3 is not enabled");
}
