fn main() {
    simple();
    structs1();
    structs2();
}

fn simple() {
    let width = 30;
    let height = 50;

    println!(
        "0: The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn structs1() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    struct Rectangle {
        width: u32,
        height: u32,
    }

    println!(
        "1: The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

fn structs2() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    println!("2: Debug `rect` is -> \n {:?}", rect);
    println!("2: PrettyPrint `rect` is -> \n {:#?}", rect);
}
