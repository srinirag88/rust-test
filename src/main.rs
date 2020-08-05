fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    another_function();
    another_function1(30);
    println!("The value of x is: {}", another_function_return_val(30));
}

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}

// Return data
fn another_function_return_val(x: i32) -> i32 {
    x + 32
}

fn ownership() {
    
}
