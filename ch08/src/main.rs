use std::collections::HashMap;
fn main() {
    // Exercise

    // 1.
    // Given vector of int, return median & mode
    // let mut v: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,2,3,4,5,8,6,8,6,9,10];
    let mut v = vec![1, 2, 3, 4, 4];
    v.sort();

    println!("avg: {:?}", calc_average(&v));
    println!("median: {:?}", calc_median(&v));
    println!("mode: {:?}", calc_mode(&v));

    // 2.
    // Convert strings to pig latin.
    // First consonant of each word is moved to the end and "ay" is added (“first” becomes “irst-fay”)
    // Words start with vowel have "hay" added to the end (“apple” becomes “apple-hay”).
    let s1 = "first";
    let s2 = "apple";
    println!("{} {}", s1, pig_latin(s1));
    println!("{} {}", s2, pig_latin(s2));

    // 3.
    // Text interface that allow user add employee to a department
    // and allow retrieve all people in department or in company sorted alphabetically
    employee_cli();
}

fn calc_average(v: &Vec<i32>) -> Option<f64> {
    if v.len() == 0 {
        return None;
    }
    let mut s = 0;
    for x in v {
        s += x;
    }
    Some(s as f64 / v.len() as f64)
}

fn calc_median(v: &Vec<i32>) -> Option<f64> {
    if v.len() == 0 {
        None
    } else if v.len() % 2 == 1 {
        Some(v[v.len() / 2].into())
    } else {
        Some(f64::from(v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0)
    }
}

fn calc_mode(v: &Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();
    let mut max_count = -1;
    let mut mode = None;
    for x in v {
        let count = map.entry(x).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            mode = Some(*x);
        }
    }
    mode
}

fn pig_latin(s: &str) -> String {
    match &s[0..1] {
        "a" | "e" | "i" | "o" | "u" => s.to_string() + "-hay",
        h => s[1..].to_string() + h + "-ay",
    }
}

#[derive(Debug)]
enum Command {
    Add(String, String),
    QueryAll(),
    Query(String),
}

use std::io;
fn employee_cli() -> () {
    println!("====================");
    println!("Employee CLI started");
    println!("====================");

    let mut db: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(err) => println!("{}", err),
            Ok(_) => (),
        };

        match parse_command(&buffer) {
            Some(cmd) => process_command(&mut db, &cmd),
            None => println!("Invalid command!"),
        }
    }
}

fn parse_command(buf: &str) -> Option<Command> {
    let words: Vec<&str> = buf.split_whitespace().collect();
    match words.get(0) {
        Some(s1) => {
            if s1.to_uppercase() == "QUERY" {
                match words.get(1) {
                    Some(s2) => Some(Command::Query(s2.to_string())),
                    None => Some(Command::QueryAll()),
                }
            } else if s1.to_uppercase() == "ADD" {
                match words.get(1..=2) {
                    Some(sl) => Some(Command::Add((&sl[0]).to_string(), (&sl[1]).to_string())),
                    None => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

fn process_command(db: &mut HashMap<String, Vec<String>>, cmd: &Command) -> () {
    match cmd {
        Command::Add(department, employee) => {
            db.entry(department.to_owned())
                .or_insert(Vec::new())
                .push(employee.to_owned());
            if let Some(l) =  db.get_mut(department) {
                l.sort()
            }
        }
        Command::Query(department) => {
            match db.get(department) {
                Some(employees) => {
                    println!("=== {} ===", department);
                    for employee in employees {
                        println!("{}", employee);
                    }
                },
                None => println!("No empolyee in {}", department),
            }
        }
        Command::QueryAll() => {
            for (department, employees) in db {
                println!("=== {} ===", department);
                for employee in employees {
                    println!("{}", employee);
                }
            }
        }
    }
}

fn lesson() -> () {
    // Vector
    {
        let v: Vec<i32> = Vec::new();
        // vec! macro allows omiting type by infering from value
        let v = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("Third: {}", third);

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("Third: {}", third),
            None => println!("No Third!"),
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        // Error by borrow checker, vector heap might be reallocated when resizing
        // println!("First is {}", first);
    }

    {
        let mut v = vec![100, 200, 300];

        // n_ref: &i32
        for n_ref in &v {
            let n_plus_one: i32 = *n_ref + 1;
            println!("{}", n_plus_one);
        }

        // n_ref: &mut i32
        for n_ref in &mut v {
            *n_ref += 50;
        }

        for n_ref in &v {
            println!("{}", n_ref);
        }
    }

    {
        // Use enum if want vec to store different type
        // but you must know the exhaustive set of types your program will get at compile time
        // Or else need to use trait
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("aa")),
            SpreadsheetCell::Float(1.23),
        ];
    }

    // String
    // String is a wrapper around Vec<u8>
    {
        let s = "init".to_string();
        let s = String::from("init");

        let mut s1 = "foo".to_string();
        s1.push_str("bar");
        println!("s1 = {}", s1);

        // Concat with format! macro
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        // Error as s1 ownership moved when init s3
        // println!("{} {} {}", s1, s2, s3);
        println!("{} {}", s2, s3);

        // format! add marco definition
        // fn add(self, s: &str) -> String {}

        // but why work when we put s2: &String in s: &str
        // Rust can coerce &String arg into &str
        // Rust uses a deref coercion, which turns &s2 into &s2[..]; details in later chapter

        // so what does let s3 = s1 + &s2; do
        // 1. add takes ownership of s1
        // 2. appends copy of content of s2 to s1
        // 3. returns ownership to s1

        // basically it uses `self` buffer and append copied content from `s`,
        // and borrow checker prevent s1 using the original buffer after ownership transfered to s3
    }

    {
        let s1 = String::from("hello");
        // Error as rust String does not implement trait Index<{integer}>
        // let h = s1[0];

        // Reason: rust string is vec<u8> vec of bytes
        // access by index does not guaratee return a valid letter as letter may be > 1 byte
        let hello = String::from("Hola"); // len = 4
        let hello = String::from("Здравствуйте"); //len=24, 12 letter

        // Use string slice to get substring
        let hello = "Здравствуйте"; // 2 byte / char
        println!("{}", &hello[0..4]);
        // Error: panicked at 'byte index 5 is not a char boundary; it is inside 'р' (bytes 4..6) of `Здравствуйте`'
        // println!("{}", &hello[0..5]);

        for c in "Зд".chars() {
            println!("{}", c);
        }

        for b in "Зд".bytes() {
            println!("{}", b);
        }
    }

    // Hash Map
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        let field_name = String::from("Cyan");
        let field_value = 20;
        scores.insert(field_name, field_value);

        // Error, ownership moved into hashmap
        // println!("{}: {}", field_name, field_value);
    }

    // Overwrite old value
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    // Insert when not present
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }

    // Update based on old value
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
