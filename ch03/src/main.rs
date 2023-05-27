fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner: {x}")
    }
    println!("Outer: {x}");

    // char is Unicode Scalar Value not ascii
    // let c = 'z';
    // let z:char = 'ℤ';
    // let heart_eyed_cat = '😻';

    // Compound type
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("{x} {y} {z}");

    let a: [i32; 5] = [1,2,3,4,5];
    let b = [3;5]; // [3,3,3,3,3];

    let ab = a == b;
    println!("{ab}");

    another_function(22);

    let y = {
        let x = 3;
        x+1
    };
    println!("y: {y}");

    let y = five();
    println!("y: {y}");

    let mut counter = 0;

    let result = loop {
        counter+=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {result}");

    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("a[{index}] = {}", a[index]);
        index+=1;
    }

    // for element in a {
    //     println!("{element}");
    // }

    for number in (1..4).rev() {
        println!("{number}");
    }


    let fb = fib(30);
    println!("{fb}");

    let q;
    if true {
        q = 1;
    } else {
        q = 2;
    }
    println!("q:{q}");
}

fn another_function(x: i32) -> () {
    println!("Another fn x: {x}");
}

fn five() -> i32 {
    5
}

fn fib(n:i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}