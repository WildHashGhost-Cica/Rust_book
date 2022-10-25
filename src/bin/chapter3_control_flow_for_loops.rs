fn main(){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the value is: {}", element);
    }
    // you don't need to add it like this (1..4) but you can and you will get warning message 
    for number in 1..4{
        println!("{}!", number);
    }
}