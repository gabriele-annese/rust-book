/*
    Ownership Rust's most unique feature, and it enables Rust to make memory safety guarantees 
    without needing a garbage collector.
    In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.
*/


fn main() {
    /*
      1) Each value in Rust has an owner.
      2) There can only be one owner at time.
      3) When the owner goes out of scope, the value will be dropped.
    */


    /* 
        variable scope example with String type
    */ 
    {
        let mut s = String::from("Hello"); // s is valid from this point forward
        s.push_str(", world!"); // push_str() appends a literal to a String () - heap allocated, growable data structure
        println!("{}", s); // This will print `Hello, world!`
    } // s is no longer valid here, scope is over. The Stirng type calles `drop` fucntion and the memory is freed


    /*
        Variable and Data Interactions with Move
    */
    let x =5;
    let y = x; // here the value of x is copied into y. Both x and y can be used independently
    println!("x = {}, y = {}", x, y); // This will print `x = 5, y = 5`
    // This appened because integers are stored on the stack and have a known, fixed size.

    // Try to do the same with String type
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // This will throw an error because s1 is no longer valid after the move to s2
    println!("{}, world!", s2); // This will print `hello, world!`

    /*
        Scope and Assignment 
    */
    let mut s3 = String::from("Hello");
    println!("{}", s3); // This will print `hello` and not Hello
    s3 = String::from("ahoy"); // This is valid because we are not trying to use s3 after it has been moved. Instead, we are assigning a new value to s3.
    println!("{}", s3); // This will print `ahoy` and not Hello

    /*
        Variables and Data Interacting with Clone
    */
    let s4 = String::from("hello im clone example");
    let s5 = s4.clone(); // This will create a deep copy of the data
    println!("s4 = {}, s5 = {}", s4, s5); // This will print `s4 = hello, s5 = hello`


    /*
        Ownership and Functions
    */
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                                // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
    println!("x = {}", x); // This will print `x = 5`


    /*
        Returns value and Scope
    */
    let s7 = gives_ownership();        // gives_ownership moves its return
                                              // value into s1
    let s8 = String::from("hello");  // s2 comes into scope
    let s9 = takes_and_gives_back(s8);      // s2 is moved into
                                           // takes_and_gives_back, which also
                                          // moves its return value into s3
    println!("s7 = {}, s9 = {}", s7, s9); // This will print `s7 = hello, s9 = hello`

}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}
fn gives_ownership() -> String{
    let some_stirng = String::from("hello");
    some_stirng
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed
