#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hodl(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    //smaller rectangle
    let rect1 = Rectangle{
        width: 20,
        height: 40,
    };
    //bigger rectagnle
    let rect2 = Rectangle{
        width: 40,
        height: 50,
    };

    println!("{:#?}", rect);

    println!("The area of rectangle is {} square pixels", rect.area());

    println!("rect can hold rect1: {}", rect.can_hodl(&rect1));
    println!("rect can hold rect2: {}", rect.can_hodl(&rect2));
}
