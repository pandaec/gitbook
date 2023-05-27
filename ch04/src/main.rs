fn main() {
    
    // Ownership Rules
    // =================================
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // From stack
    let _s = "hello";
    // From heap
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s1);
    let s2 = s1;

    // cannot borrow `s2` as mutable, as it is not declared as mutable
    // s2.push_str(", world!!!!");

    {
        // borrow of moved value: `s1`
        // let s1 = String::from("hello");
        // let s2 = s1;
        // println!("s1 = {}, s2 = {}", s1, s2);

        // Fixed
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        // Require clone because is data on heap
        // Data on stack doesn't have same problem. since copy by value
        // All because it has Trait: Copy
        // Most primitive types and tuple derives Copy
        let x = 5;
        let y = x;
        println!("x={},y={}", x , y);

        {
            let _s1 = gives_ownership();
            let _s2 = String::from("hello");
            let _s3 = takes_an_gives_back(s2);
            // Error s2 String::from("hello") ownership is at s3
            // println!("{}", _s2);
        }

        {
            let s1 = String::from("hello");
            let len = calculate_len(&s1);
            println!("Length of {} is {}", s1, len);
        }

        {
            // Reference are immutable by default
            // let _s = String::from("hello");
            let mut _s = String::from("hello");
            change(&mut _s);
        }

        {
            // Mutable reference restriction:
            // Only can have 1 reference to it

            // let mut s = String::from("Hello");
            // let r1 = &mut s;
            // let r2 = &mut s;
            // println!("{},{}", r1, r2);

            // Invalid even if other references are immutable
            // let mut s = String::from("Hello");
            // let r1 = &s;
            // let r2 = &s;
            // let r3 = &mut s;
            // println!("{},{},{}", r1, r2, r3);
        }

        {
            let mut s = String:: from("Hello");
            
            let r1 = &s;
            let r2 = &s;
            // No erorr here as r1,r2 is not used after mut reference
            println!("r1 {}, r2 {}", r1, r2);
            
            let r3 = &mut s;
            println!("{}", r3);
            // Error here as immutable reference is read after got mut reference
            // println!("r1 {}, r2 {}", r1, r2);
        }

        {
            let s = String::from("hello world");

            // [0..len] by default
            let hello = &s[..5];
            let world = &s[6..];
        }

        {
            // Error; cannot borrow mut ref as immut ref is used after
            let mut s = String::from("hello world");
            let word = first_word(&s);
            s.clear();
            // println!("first word is {}", word);
        }

        {
            // String Literal = slice
            let s = String::from("Hello world");
            let word = first_word(&s[0..6]);
            let word = first_word(&s[6..]);
            let word = first_word(&s);
            let word = first_word("Hello abc");
            // let word = first_word(s);
        }
    }

    fn gives_ownership() -> String {
        let s = String::from("yours");
        s
    }
    fn takes_an_gives_back(a:String) -> String {
        a
    }

    fn calculate_len(s: &String) -> usize {
        s.len()
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }

    // &str is type of "string slice"
    // &str works on both slice (&str) and string ref (&String)
    // fn first_word(s: &String) -> &str {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for(i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}
