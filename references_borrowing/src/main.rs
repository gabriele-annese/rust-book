fn main() {
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);
    println!("The length of '{s1}' is {len}");

    // Mutable references
    let mut s2 = String::from("hello");
    change_string(&mut s2);
    println!("{s2}")
}

fn change_string(s: &mut String){
    s.push_str(", world");
}

fn calculate_lenght(s: &String) -> usize{ // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.
