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

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
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

    let rect3 = Rectangle::square(25);

    println!("{:#?}", rect);

    println!("The area of rectangle is {} square pixels", rect.area());

    println!("rect can hold rect1: {}", rect.can_hodl(&rect1));
    println!("rect can hold rect2: {}", rect.can_hodl(&rect2));
    println!("The area of rectangle is {} squear pixels", rect3.area());
}
