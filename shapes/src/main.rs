fn main() {
    simple();
    structs1();
    structs2();
    structs3();
    structs4();
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

fn structs3() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // self/Self is a magic word (not just convention)
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "3: The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn structs4() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let sqr: Rectangle = Rectangle::square(15);

    println!("4: The area of the square is {} square pixels.", sqr.area());
    println!(
        "4: The rectangle can hold the square? {}",
        rect.can_hold(&sqr)
    )
}
