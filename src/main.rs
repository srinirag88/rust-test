// Using standard imports

mod collections;
mod library;
use collections::*;
use library::*;
use std::string::String;

fn main() {
    another_function();
    another_function1(30);
    println!("The value of x is: {}", another_function_return_val(30));
    println!("{}", hello_string(String::from("Hello")));
    scope_test();
    mem_allocation();
    clone_data();
    move_reference();
    fn_ownership();
    stuct_use();
    new_libraray_function(12);
    vector();
    vector_iterate();
    hash_map();
    create_file_write_data(String::from("Srini"), String::from("First data")).expect("Failed");
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

// Process and return a data. Return data is optional
fn hello_string(x: String) -> String {
    format!("{}{}", x, "Hello ")
}

// Scope understanding..
// So the variable is not carried into the scope and variable inside scope is not exposed out
fn scope_test() {
    let x = "test";
    {
        let x = "mine";
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
}

// Mutable types are allocated on the heap and
fn mem_allocation() {
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("The value of x is: {}", s);
}

// Rust prevents you from using the invalidated reference
// Which means when u move s1 to s2 u cannot use s1 to refer the original object

fn move_reference() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
}

//Cloning a heap data to a different reference
fn clone_data() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// When a reference is sent to another function reference cannot be used.
// But we can declare the same variable as it has been destroyed in the process
// of sending to a a different scope
fn fn_ownership() {
    let s1 = String::from("hello");
    take_ctrl(s1);
    let s1 = String::from("hello");
    println!("s1 = {}", take_ctrl_return(s1));
}

fn take_ctrl(mut s: String) {
    s.push_str(" mutated");
    println!("{}", s);
}

fn take_ctrl_return(mut s: String) -> String {
    s.push_str(" mutated and returned back");
    s
}

fn stuct_use() {
    let u = build_user(
        String::from("srinirag88@gmail.com"),
        String::from("srinirag88"),
    );
    println!("{} and {} ", u.email, u.username);
    println!("{} ", u.email_data());
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct TupleColor(i32, i32, i32);

fn ret_tuple() -> TupleColor {
    TupleColor(0, 0, 0)
}

// Impl can be used add functions which can access the data of the structs
impl User {
    fn email_data(&self) -> String {
        format!("{}{}", self.email, " Hello ")
    }
}

enum Task {
    First,
    Second,
    Thrid,
    Fourth,
}

fn mapping_enum(task: Task) -> String {
    match task {
        Task::First => String::from("First"),
        Task::Second => String::from("Second"),
        Task::Thrid => String::from("Thrid"),
        Task::Fourth => String::from("Fourth"),
    }
}
