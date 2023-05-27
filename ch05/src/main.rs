struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

struct P {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self = self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions
    // functions parameter not starting with &self; often used for constructor called "new()"
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    {
        let mut user1 = User {
            email: String::from("someone@email.com"),
            username: String::from("username123"),
            active: true,
            sign_in_count: 1,
        };
        user1.email.push_str(".hk");

        let user2 = build_user(String::from("user2@mail.com"), String::from("user2"));

        // If move heap data using this syntax, then will also get old user's memebership
        // However if just copy stack data then still able to use old user
        let user3 = User {
            email: String::from("user3@mail.com"),
            username: String::from("user3"),
            ..user2
        };
    }

    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        let subject = AlwaysEqual;
    }

    {
        let mut p = P { x: 1, y: 2 };
        let x = &mut p.x;
        let y = &mut p.y;
        *x += 1;
        *y += 1;
        println!("{} {}", p.x, p.y);
    }

    {
        let rect1 = Rectangle {
            width: 50,
            height: 50,
        };

        println!("Area of rectangle {:?} is {}", rect1, area(&rect1));
        println!("Area of rectangle {:?} is {}", rect1, rect1.area());
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
        println!(
            "Can rect1 hold rect3? {}",
            Rectangle::can_hold(&rect1, &rect3)
        );

        let square1 = Rectangle::square(3);
        println!("Square1 {:?}", square1);
    }

    {
        let r = &mut Box::new(Rectangle {
            width: 1,
            height: 2,
        });
        let area1 = r.area();
        let area2 = Rectangle::area(&r);
        assert_eq!(area1, area2);
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
