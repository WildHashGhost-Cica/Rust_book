struct Rectangle {
    width: u32, 
    height: u32, 
}

fn main(){
    /*let width1 = 30;
    let height1 = 50;*/
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("The area of the rectangel is {} square pixels",area(width1, height1) );
    println!("The area of the rectangel is {} square pixels", area(&rect));
}
/*
fn area( width: u32, height:u32) -> u32 {
    width *  height
}*/
fn area(rectangel: &Rectangle) -> u32 {
    rectangel.width * rectangel.height
}
