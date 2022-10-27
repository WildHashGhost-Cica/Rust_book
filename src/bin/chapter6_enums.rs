#[derive(Debug)]
enum IpAddrKind{
    /*
    V6,
    V4,
    */
    V6(String),
    /*
    V4(String),*/
    V4(u8, u8, u8, u8)
}
enum Message {
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn some_function(){
        println!("Let's Get Rusty!");
    }
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
/*
struct QuitMessage;
struct MoveMessage{
    x: i32, 
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
*/
fn main(){
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;*/
    /*
    let localhost = IpAddr{
        kind: IpAddrKind::V4;
        address: String::from("127.0.01");
    };*/
    /*
    let localhost = IpAddrKind::V4(String::from("127.0.01"));*/
    let localhost = IpAddrKind::V4(127,0,0,1);
    println!("localhost {:?}", localhost);
}

fn route(ip_kind: IpAddrKind){

}