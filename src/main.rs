use std::fs::File;
use std::io::Error;
struct User {
    username: String,
    email: String,
    active: bool,
}

enum Message {
    Quite,
    Write(String),
    Move { x: i32, y: i32 },
}

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("the value of x is: {}", x);
    let mut y = 5;
    println!("the value of y is :{}", y);
    y = 20;
    println!("the new value of y is :{}", y);
    let word = 'a';
    let pi = 3.123;
    let is_active = true;

    // if else
    if y < 5 {
        println!("number is less than 20");
    } else if y == 20 {
        println!("equal");
    } else {
        println!("number is greater than 20");
    }

    //loops {loop,while,for}
    // loop {
    //     println!("this will loop forever");
    // }

    let mut counter = 0;
    while counter <= 5 {
        println!("Count: {}", counter);
        counter += 1;
    }

    for i in 0..5 {
        println!("number {}", i);
    }
    //###Borrowing ###:
    // When a variable is borrowed, you can either have:
    // Immutable references (&T) which allows multiple borrows.
    // Mutable references (&mut T) which allows only one borrow at a time.

    let s = String::from("hello");
    let len = calculate_length(&s); // Borrowing `s` as an immutable reference
    println!("The length of '{}' is {}.", s, len);

    let mut s2 = String::from("hello");
    change(&mut s2); // Mutable reference
    println!("Changed value: {}", s2);

    //Ownership
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of s1 is moved to s2
                 // println!("{}", s1);  // This would cause an error because s1 is no longer valid
    println!("{}", s2); // s2 is the owner now
    let length = calculate_length(&s2);
    println!("the length of s2 is {} {}", s2, length);

    let result = add(10, 20);
    println!("result {}", result);

    //Structs
    let user1 = User {
        username: String::from("sreekanth"),
        email: String::from("sreekanth3265@gmail.com"),
        active: true,
    };
    println!(
        "user details {} {} {}",
        user1.username, user1.email, user1.active
    );

    // ENUMS & PATTERN MATCHING
    let msg = Message::Move { x: (20), y: (30) };

    match msg {
        Message::Quite => println!("Quite message"),
        Message::Write(text) => println!("Write message {}", text),
        Message::Move { x, y } => println!("Move message ({},{})", x, y),
    }

    //  Error Handling: Option and Result
    //    Rust doesn’t use exceptions. Instead, it uses the Result and Option types for error handling.

    // Option:
    // Some: Represents a value.
    // None: Represents no value.
    let divide_result = divide(50.0, 2.0);
    match divide_result {
        Some(value) => println!("Result {}", value),
        None => println!("canot divide by zero"),
    }
    let file = File::open("sample.txt");
    match file {
        Ok(f) => println!("file opened successfully :{:?}", f),
        Err(e) => println!("file not exist :{:?}", e),
    }

    // Arrays
    let mut arr = [10, 20, 30, 40, 50];

    // Accessing elements
    println!("First element: {}", arr[0]);

    // Modifying elements
    arr[2] = 35;
    println!("Modified array: {:?}", arr);

    // Getting length
    println!("Array length: {}", arr.len());

    // Iterating through array

    for element in arr.iter() {
        println!("Element:{}", element)
    }

    // Array slicing
    let slice = &arr[2..4];
    println!("Array slice: {:?}", slice);

    // Vectors
    let mut vec1: Vec<i32> = Vec::new(); // create an empty vector
    vec1.push(1);
    vec1.push(1);
    vec1.push(1);
    vec1.push(1);
    println!("elements {:?}", vec1);
    let vec2 = vec![1, 2, 1];
    println!("verctor {:?}", vec2);
    println!("First element: {}", vec2[0]);
    vec1.push(40); // Add an element
    println!("After push: {:?}", vec1);

    vec1.pop(); // Remove the last element
    println!("After pop: {:?}", vec1);
    match vec2.get(2) {
        Some(value) => println!("value {}", value),
        None => println!("no value"),
    }

    // mut
    let mut vec3 = vec![1, 2, 3, 4, 5];
    vec3[2] = 10;
    println!("vector 3 {:?}", vec3);

    // Immutable reference - read-only iteration
    for val in &vec1 {
        println!("value {}", val);
    }
    // Mutable reference - allows modification of elements during iteration
    let mut vec_mut = vec![1, 2, 3];

    for val in &mut vec_mut {
        *val *= 2;
    }
    println!("modified vector{:?}", vec_mut);

    // slicing vector
    let vector = vec![1, 2, 3, 4, 51, 6, 7];
    let slice = &vector[1..5]; // Borrowing a slice from the vector
    println!("Slice: {:?} {}", slice, vector[5]);

    //Lifetimes:
    // ensure that references are valid as long as needed but no longer.
    // Rust uses lifetime annotations ('a) to track how long references should last.
    let string1 = String::from("longest string");
    let string2 = String::from("short string");
    let res = longest(&string1, &string2);
    println!("longest word {}", res)
}

fn calculate_length(s: &String) -> usize {
    s.len() // We can use `s` without taking ownership
}
fn change(s: &mut String) {
    s.push_str(", world"); // `s` is mutable, so we can modify it
}
fn add(x: i32, y: i32) -> i32 {
    x + y // Implicit return without a semicolon
}
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
