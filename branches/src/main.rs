

/// Function demonstrating nested loops with labels
fn multpy_loops(stop_loop: i32){
    
    if stop_loop < 0 {
        println!("stop_loop must be non-negative");
        return;
    }

    let mut count = 0;
    'counting_loop: loop{
        println!("count = {count}");
        let mut remaining = 10; 
        loop{
            if remaining == 9 {
                break;
            }
            if count == stop_loop{
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

/// Function to check divisibility by 4, 3, or 2
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

fn main() {
    let numbers: [i32; 4] = [8, 9, 10, 11];

    for n in numbers {
        println!("[+] Checking number: {n}");
        check_divisibility(n);
    }

    // Reuturn a value from a loop expression
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 1000 {
            break counter * 2;
        }
    };

    println!("The loop result is: {loop_result}");


    // Demonstrate nested loops with labels
    let stop_loop = 2;
    multpy_loops(stop_loop);

    /* 
        For loop with Range and rev()
        The last number in the range is exclusive, in this case 10 is excluded
    */ 
    for number in (1..10).rev(){
        println!("[+] {number}!");
    }
}