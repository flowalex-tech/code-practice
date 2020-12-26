const PI: f64 = 3.14159265359;

fn main() {
    println!("{}, {}!", "Hello", "world"); // Hello, world!
    println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    println!("{greeting}, {name}!", greeting="Hello", name="world"); // Hello, world!

    println!("{:?}", [1,2,3]); // [1, 2, 3]
    println!("{:#?}", [1,2,3]);
    /*
        [
            1,
            2,
            3
        ]
    */

    // 🔎 The format! macro is used to store the formatted string.
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!

    // 💡 Rust has a print!() macro as well
    print!("Hello, world!"); // Without new line
    println!(); // A new line

    print!("Hello, world!\n"); // With new line

    let x: f64 = -20.48; // float
    let x: i64 = x.floor() as i64; // int
    println!("{}", x); // -21

    let s: &str = "hello"; // &str
    let s: String = s.to_uppercase(); // String
    println!("{}", s) // HELLO

    println!("π value is {}", PI);

}
