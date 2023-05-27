enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    {
        let home = IpAddrKind::V4(127, 0, 0, 1);
        let loopback = IpAddrKind::V6(String::from("::1"));

        let m = Message::Write(String::from("Hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_char = Some('e');

        let five = Some(5);
        let six = plus_one(five);
        let seven = plus_one(None);
    }

    // Correct
    {
        let opt: Option<String> = Some(String::from("Hello world"));

        match opt {
            Some(_) => println!("Some!"),
            None => println!("None!"),
        };

        println!("{:?}", opt);
    }

    // Error!
    {
        // let opt: Option<String> = Some(String::from("Hello world"));

        // match opt {
        //     // diff
        //     Some(s) => println!("Some!"),
        //     // opt move to s, and loses R O
        //     None => println!("None!")
        // };

        // println!("{:?}", opt);
    }

    // idiomatic solution
    {
        let opt: Option<String> = Some(String::from("Hello world"));

        match &opt {
            // s has type &String now
            Some(s) => println!("Some!"),
            None => println!("None!"),
        };

        println!("{:?}", opt);
    }
    // ch6.3
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("Max is {}", max),
            _ => (),
        }
    }
    // using if let
    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("If let Max is {}", max)
        }
    }

    // Pattern matching
    {
        let mut count = 0;
        let coin = Coin::Quarter(State::B);
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
    }
    // vs if let else
    {
        let mut count = 0;
        let coin = Coin::Quarter(State::B);
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    A,B,C
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
