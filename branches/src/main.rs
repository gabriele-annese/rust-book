fn main() {
    let numbers: [i32; 4] = [8, 9, 10, 11];

    for n in numbers {
        println!("[+] Checking number: {n}");
        check_divisibility(n);
    }
}

fn check_divisibility(number: i32) {
    if number % 4 == 0 {
        println!("The number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number {number} is divisible by 2");
    } else {
        println!("The number {number} is not divisible by 4, 3, or 2");
    }
}
