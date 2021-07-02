fn main(){
let mut a = 0;

loop {
    if a == 5 {
        break;
    }
    println!("{:?}", a);
    a = a + 1;
}

while a != 5 {
    println!("{:?}", a);
    a = a + 1;
}
}
