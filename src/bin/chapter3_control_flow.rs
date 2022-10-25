fn main(){
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    }else if number < 22 {
        println!("second condition was true");
    }else{
        println!("condition was false");
    }

    loop {
        println!("again!");
        break;
    }
    let mut counter = 0; 

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
       
    };
    println!("the result is: {}",result);

    let mut number1 = 3;
    while number1 != 0 {
        println!("{}", number1);
        number1 -= 1;
    }
    println!("Liftoff");
}