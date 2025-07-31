// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    // #[macro_export]          // Export the macro to crate root namespace
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;    // The second method
}

fn main() {
    macros::my_macro!();
}
