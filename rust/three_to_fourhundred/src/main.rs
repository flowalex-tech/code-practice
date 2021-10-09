extern crate regex;
use regex::Regex;

fn main() {
    // If you wrote all the numbers from 300 to 400 on a piece of paper, how many times would you have written the number 3?
    //let first_expression = Regex::new(r"\d[3]\d[0-9]\d[0-9]").unwrap();
    //let second_expression = Regex::new(r"\d[3]\d[3]\d[0-9]").unwrap();
    //let third_expression = Regex::new(r"\d[3]\d[0-9]\d[3]").unwrap();

    let nums = "300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400";

    let mut count = 0;
    println!("{}", count);
    // for cap in first_expression.captures_iter(nums){
    //     println!("{}", &cap[1]);
    //     count += 1;
    //     println!("{}",count);
    // }
    let my_captures: Vec<&str> = Regex::new(r"\d[3]\d[0-9]\d[0-9]").unwrap().find_iter(nums).map(|x| x.as_str()).collect();
    println!("{:?}", my_captures[1]);
    count += 1;
    println!("{}", count)

}
