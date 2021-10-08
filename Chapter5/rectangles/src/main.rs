fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.", 
        area(length1, width1)
    );

    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}