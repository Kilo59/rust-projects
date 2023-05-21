fn main() {
    simple();
    with_structs();
}

fn simple() {
    let width = 30;
    let height = 50;

    println!(
        "1: The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn with_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    print!(
        "2: The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    struct Rectangle {
        width: u32,
        height: u32,
    }
}
