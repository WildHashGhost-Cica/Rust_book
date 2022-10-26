#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main(){
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:#?}", rect);

    //println!("The area of the rectangel is {} square pixels", area(&rect));
    println!("The area of rectangle is {} square pixels", rect.area());
}
/*
//implament it in Rectangle
fn area(rectangel: &Rectangle) -> u32 {
    rectangel.width * rectangel.height
}
*/