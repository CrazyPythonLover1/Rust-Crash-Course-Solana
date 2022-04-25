fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 100;

    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = -10;

    // float is used for decimals
    let float: f32 = 1.33;

    println!("unsigned: {} signed: {} float: {} ", unsigned, signed, float);

    // char - can only be
    let letter = "Mainul Islam Faruqi";
    let emoji = "\u{1F600}"; // :D
    println!("letter: {} emoji: {}", letter, emoji);

    let is_true: bool = true;

    println!("isTrue: {}", is_true);
    println!("Hello, world!");

    // Arrays 
    let arr: [u8; 4] = [1, 4, 6, 7];
    let other_arr: [u8; 5] = [200; 5];

    println!("index: {} length: {}", arr[0], other_arr.len());

    // print the whole array 
    println!(" {:?} ", other_arr);

    let tuple: (u8, bool, f32) = (50, true, 39.5);
    let tuple2 = (3, 5);

    println!("first {} second {} third {} ", tuple.0, tuple.1, tuple.2);
    println!(" {:?} ", tuple2);

    let (a, b, c) = tuple;

    // Destructuring
    println!("first {} second {} third {}", a, b, c);

    println!("{}", is_even(89));
    pub fn is_even(num: u8) -> bool {
        let digit: u8 = num % 2;
        digit == 0 // return bool
    }

}
