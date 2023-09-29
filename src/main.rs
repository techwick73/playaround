mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;

const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}", OUR_COURSE);

    // Stack
    let x: i32;
    x = 2;
    println!("x on the stack is {}", x);

    let y = 4;
    println!("y on the stack is {}", y);

    // For loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i)
        } else {
            println!("y is {}", y);
        }
    }

    // Mutation
    let mut z = 5;
    println!("z was {}", z);
    z = 10;
    println!("z is now {}", z);

    let freezing_temp = -2.4;
    println!("freezing_temp is  {}", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 == 0;
    println!("is_zero_remainder is  {}", is_zero_remainder);

    let my_char = 'z';
    println!("my_char is {}", my_char);

    let emoji_car = '😀';
    println!("emoji_car is {}", emoji_car);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);

    let name: &str = "JP Colson";
    println!("string literal name on stack is {:?}", name);

    let dynamic_name = String::from("JP");

    println!("dynamic string name in heap is {:?}", dynamic_name);
    println!(
        "dynamic string name in heap at memory location {:p}",
        &dynamic_name
    );

    let dynamic_name = name.to_string();
    println!("dynamic_name is {:?}", dynamic_name);

    let dynamic_name = "JP Colson".to_string();
    println!("dynamic_name is {:?}", dynamic_name);

    let str_slice = &dynamic_name[0..5];
    println!("str_slice {:?}", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('.');

    println!("chars is {:?}", chars);
    dbg!(&chars);

    let removed_char = chars.pop().unwrap();
    println!("removed_char is {:?}", removed_char);
    println!("chars is {:?}", chars);

    // chars.iter().for_each(|c| print!("{}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect();
    dbg!(&collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            print!(" ,world!");
        }
    }

    // Closures
    let num = 5;
    let add_num = |x: i32| x + num;
    let new_num = add_num(7);
    dbg!(new_num);

    //Number Literals (from Rust Book)
    println!("Big Number is {}", 98_222_000);
    println!("Hex Number is {}", 0xff);
    println!("Octal Number is {}", 0o77);
    println!("Binary Number is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');

    // Raw  - String Literal
    let text: &str = r#"{\"message" : "Rust is awesome"}"#;
    dbg!(text);

    //Binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);

    println!("a's value is {:08b}", a);
    println!("b's value is {:08b}", b);

    // Logic Gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT {:08b}", !a);

    //Bitwise Operations
    println!("a << 1 {:08b}", a << 1);
    println!("a << 1 {}", a << 1);
    println!("a >> 1 {:08b}", a >> 1);
    println!("a >> 1 {}", a >> 1);

    // Little Endian or Big Endian
    let n: u16 = 0x1234;
    println!("n is {:?}", n);

    let big_endian = n.to_be_bytes();
    let little_endian = n.to_le_bytes();

    println!("n in big endian {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!(
        "n in little endian {:02X}{:02X}",
        little_endian[0], little_endian[1]
    );
}
