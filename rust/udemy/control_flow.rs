let a = 99;

if a > 99 {
    if a > 200 {
        println!("Huge Number");
    } else {
    println!("Big Number");
    }
} else {
    println!("Small Number");
}

if a > 200 {
    println!("Huge Number");
else if a > 99 {
    println!("Big Number");
} else
    println!("Small Number");
}
