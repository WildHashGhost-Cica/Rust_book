fn main(){
    /*enum Option<T>{
        Some(T),
        None,
    }*/
    /*
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;*/
    let x: i8 = 5; //integer
    let y: Option<i8> = Some(5); //option integer
    let z: Option<i8> = None;
    //let sum = x + y; we got error because different type
    let sum = x + y.unwrap_or(0);
    println!("x + y = {}", sum);
    let sum1 = x + y.unwrap_or(0) + z.unwrap_or(0);
    println!("x + y + z = {}", sum);

}