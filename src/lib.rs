// #[cfg(feature = "examples")]
// pub use workspace_1;
// #[cfg(feature = "examples")]
// pub use workspace_2;
// #[cfg(feature = "examples")]
// pub use workspace_3;

pub mod api {
    // #[cfg(feature = "workspace_1")]
    pub struct DataOne(workspace_1::DataOne);

    // #[cfg(feature = "workspace_1")]
    impl DataOne {
        pub fn new(value: i32) -> Self {
            Self(workspace_1::DataOne { value })
        }

        pub fn process(&self) -> i32 {
            workspace_1::process_one(&self.0)
        }
    }

    // #[cfg(feature = "workspace_2")]
    pub struct DataTwo(workspace_2::DataTwo);

    // #[cfg(feature = "workspace_2")]
    impl DataTwo {
        pub fn new(base: DataOne, factor: f64) -> Self {
            Self(workspace_2::DataTwo {
                base: base.0,
                factor,
            })
        }

        pub fn process(&self) -> f64 {
            workspace_2::process_two(&self.0)
        }
    }

    // #[cfg(feature = "workspace_3")]
    pub struct DataThree(workspace_3::DataThree);

    // #[cfg(feature = "workspace_3")]
    impl DataThree {
        pub fn new(base: DataOne, text: String) -> Self {
            Self(workspace_3::DataThree { base: base.0, text })
        }

        pub fn process(&self) -> String {
            workspace_3::process_three(&self.0)
        }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn run_all_greetings() -> Vec<String> {
        let mut greetings = Vec::new();

        // #[cfg(feature = "workspace_1")]
        greetings.push(workspace_1::greet_one());

        // #[cfg(feature = "workspace_2")]
        greetings.push(workspace_2::greet_two());

        // #[cfg(feature = "workspace_3")]
        greetings.push(workspace_3::greet_three());

        greetings
    }
}
