// The constnat can be declare in the global scope of the program
const CONST_STRING: &str = "context of constant string";

fn main() {
    /**
     * The variable on Rust are immutable by default.
     * To declare a mutable variable we need to add the "mut" key word after let
     */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant in Rust
    println!("This instance is my constant: {CONST_STRING}");

    /**
     * In rust u can create a "shadown" variable.
     * For example if u create a no mutable variable A
     * let A = 5;
     * And after create another variable with the same name
     * let A = "I have change the type of my variable"
     */
    let shadow = 5;
    println!("This is a value of shadow variable: {shadow}");
    {
        let shadow = shadow + 5;
        println!("This is a value of shadow variable in the scope: {shadow}");
    }
    let shadow = "I like coffee";
    println!("This is a value of shadow variable now: '{shadow}'");
}
