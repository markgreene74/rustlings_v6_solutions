fn main() {
    my_macro!();
}

// TODO: Fix the compiler error by moving the whole definition of this macro.
// Move the macro up the file, before main(). Alternatively, use macro_export to
// declare the macro in the root scope, see
// https://doc.rust-lang.org/reference/macros-by-example.html#path-based-scope
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
