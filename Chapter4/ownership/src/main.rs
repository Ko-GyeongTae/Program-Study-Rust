fn main() {
    append_string();

    clone_string();

    stack_clone();

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takens_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    
    let s1 = String::from("hello");
    let len = calculate_length1(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
}

fn append_string() {
    let mut s = String::from("Hello");
    
    s.push_str(", World!");

    println!("{}", s);
}

fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_clone() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takens_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length1(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}