#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

fn main(){
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:#?}", rect);

    println!("The area of the rectangel is {} square pixels", area(&rect));
}

fn area(rectangel: &Rectangle) -> u32 {
    rectangel.width * rectangel.height
}
