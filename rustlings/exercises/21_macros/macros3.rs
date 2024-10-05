// TODO: Fix the compiler error without taking the macro definition out of this
// module.
// Solution 1: add macro_use here
#[macro_use]
mod macros {
    // Solution 2: add macro_export here
    // #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
