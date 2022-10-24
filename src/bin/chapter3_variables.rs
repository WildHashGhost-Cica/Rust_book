fn main(){
    let mut x = 5;

    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    let y = 15;
    println!("Y is = {}",y);
    //shadowing
    let y: &str = "six";
    println!("y is now = {}",y);
}