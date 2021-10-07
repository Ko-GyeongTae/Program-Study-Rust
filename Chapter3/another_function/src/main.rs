fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(1);

    let x = five();
    println!("The value of x is : {}", x);
    
    let x = plus_one(five());
    println!("The value of x is : {}", x);
} 

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of argument x is : {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}