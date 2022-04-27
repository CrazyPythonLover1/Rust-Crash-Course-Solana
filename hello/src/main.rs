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

    let arr2 = [0, 1, 2, 3, 4, 5, 6, 7];
    let slice = &arr2[0 .. 5]; // [1,2] don't know the length
    
    borrowing_slice(arr2, slice);

    // String Data type 
    let text: &str = "Blockchain Developer";
    let mut string: String = String::from("Blockchain Developer");

    let slice = &string[.. 5];
    println!("slice: {}", slice);
    println!("slice len: {}", slice.len());

    string.push('1');
    string.push_str("! Bob");

    string = string.replace("Blockchain", "Software");

    println!("{}", string);

    // If else statement 
    let n = 0;
    if n > 0 {
        println!("Greater than 0");
    } else if n < 0 {
        println!("Less than 0");
    } else {
        println!("is 0")
    }

    // For loop iteration
    for i in 0..11 {
        println!("{}", i);
    }

    // While loop iteration
    let mut i = 0;
    while i < 5 {
        // println!("While Loop");
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("exit");
            break
        }
    }

    // match 
    let x = 1;

    match x {
        0 => println!("zero"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        5 => println!("five"),
        _ => println!("Default, something else"),
    }

    // use of struct 
    let name = String::from("bird");
    let bird = Bird {name, attack: 5};
    bird.print_name();
    bird.print_attack();
    println!(" {} {} ", bird.can_fly(), bird.is_animal());
}

// Struct types
struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name)
    }

    fn print_attack(&self) {
        println!("{}", self.attack)
    }
}

// trait

impl Animal for Bird {
    fn can_fly(&self) -> bool {
     true   
    }
    fn is_animal(&self) -> bool {
        false
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

fn borrowing_slice(arr: [u8; 8], slice: &[u8]) {
    println!(" {:?} ", arr);
    println!(" {:?} ", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
