fn main() {
    simple();
    structs1();
    structs2();
    structs3();
    structs4();

    enum_vs_structs1();
    enum_vs_structs2();
    enum_vs_structs3();

    enums1();
    option1();
    match1();
    match2();
    match_w_option();
    match_catch_all();
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

fn enum_vs_structs1() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    fn route(_ip_kind: IpAddrKind) {}

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn enum_vs_structs2() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
}

fn enum_vs_structs3() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}

fn enums1() {
    #[derive(Debug)]
    enum Message {
        Quite,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Move { x: 5, y: 7 };
    m1.call();
    println!("{:?}", m2);
}

fn option1() {
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn match1() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let value = value_in_cents(Coin::Dime);
    println!("Cents: {}", value);

    fn value_in_cents_verbose(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => {
                println!("3 more for a Dollar!");
                25
            }
        }
    }

    value_in_cents_verbose(Coin::Penny);
    value_in_cents_verbose(Coin::Nickel);
    value_in_cents_verbose(Coin::Dime);
    value_in_cents_verbose(Coin::Quarter);
}

fn match2() {
    #[derive(Debug)] // for inspecting state
    enum UsState {
        Alabama,
        Alaska,
        Arkansas,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn match_w_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // matches must cover all possibilities
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn match_catch_all() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // this is the catch all
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let dice_roll2 = 5;
    match dice_roll2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch all when we don't care about the value
        _ => reroll(),
    }

    let dice_roll3 = 8;
    match dice_roll3 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch all but with a no-op
        _ => (),
    }
}
