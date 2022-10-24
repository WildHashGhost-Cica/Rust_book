fn main(){
    my_function();
    my_function1(13, 21);
    let sum1 = my_function1(15, 8);
    println!("sum1 = {}", sum1);
}

fn my_function() {
    println!("Another function");
}

fn my_function1(x:i32, y:i32) -> i32 {
    println!("my_function1: {} {}", y, x);
    println!("x from my_funciton1: {}", x);
    let sum = x + y; //we can leave this line and the next one too no ; by the end
    return sum; //we cant leave this line or just write only sum no ; by the end 

    


}

