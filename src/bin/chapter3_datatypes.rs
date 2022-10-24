fn main(){
    //Integers
    let a = 98_222; //Decimal
    let b = 0xff;   //Hex
    let c =0o77;    //Octal
    let d = 0b1111_0000;    //Binary
    let e = b'A';   //Byte (u8 only)

    let f: u8 = 255;

    //Floating-point numbers

    let f1:f64  = 2.0;
    let g: f32 = 3.0;

    //addition
    let sum = 5 + 10;
    println!("sum: {}",sum);
    let difference = 95.5 - 4.3;
    println!("difference: {}",difference);
    let product = 4 * 30;
    println!("Product: {}",product);
    let quotient = 56.7 / 32.2;
    println!("quotient: {}",quotient);
    let remainder = 43 % 5;
    println!("Remainder: {}",remainder);

    //Booleans
    let t = true;
    let f = false;

    //Character

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    
    //Compound Types - Tuple 
    let tup:(&str, i32) = ("Hello Rust", 100_000);
    println!("{}", tup.0);
    println!("{} {}", tup.1, tup.0);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;
    println!("Sub_count : {}", sub_count);
    println!("channel: {}", channel);

    let error_codes:[i32;3] = [200, 404, 500];
    let not_found = error_codes[1];
    let byte:[i32;8] = [0;8];

    println!("not found: {}", not_found);
}