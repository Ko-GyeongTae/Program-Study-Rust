fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(1);

    let x = five();
    println!("The value of x is : {}", x);
    
    let x = plus_one(five());
    println!("The value of x is : {}", x);

    controller_func();
    whileloop_func();

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is : {}", number);

    rangebased_func();
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

fn controller_func() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn whileloop_func() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn rangebased_func() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("the value is: {}", number);
    }
}