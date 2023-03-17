fn main() {
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    ley (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let user_info = ("Emma", 20);
    println!("{:?}, {:?}", user_info.0, user_info.1);
    let (name, age) = ("Emma", 20);;
    println!("{:?}, {:?}", name, age);
    //                                      0     1    2      3        4
    let favorites = ("blue", 10, "MN", "pizza", "battlebots");
    let state = favorites.2;
    let color = favorites.0;
}
