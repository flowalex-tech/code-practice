fn nonogram(v: &[i8]) -> Vec<usize> {
    let mut ans = Vec::<usize>::new();

    let mut count = 0;

    for i in v {
        if *i == 1 {
            count += 1;
        } else if count != 0 {
            ans.push(count);
            count = 0;
        }
    }
    if count != 0 {
        ans.push(count);
    }
    ans
}

fn main() {
    let result = nonogram(&[0, 0, 0, 0, 0]);
    println!("{:?}", result);
    let result = nonogram(&[1, 1, 1, 1, 1]);
    println!("{:?}", result);
    let result = nonogram(&[0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]);
    println!("{:?}", result);
    let result = nonogram(&[1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0]);
    println!("{:?}", result);
    let result = nonogram(&[0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1]);
    println!("{:?}", result);
    let result = nonogram(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
    println!("{:?}", result);
}

//mod tests
    //use crate::*;
//    fn it_works() {
//        assert_eq!(nonogram(&[0, 0, 0, 0, 0]), []);
//        assert_eq!(nonogram(&[1, 1, 1, 1, 1]), []);
//        assert_eq!(nonogram(&[0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1]), [5]);
//        assert_eq!(nonogram(&[1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0]), [5, 4]);
//        assert_eq!(nonogram(&[0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1]), [2, 1, 3]);
//        assert_eq!(nonogram(&[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), [1, 1, 1, 1, 1, 1, 1, 1]);
//    }
