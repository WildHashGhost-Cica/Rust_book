struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn main(){
    let mut user1 = User{
        email: String::from("ghost@gmail.com"),
        username: String::from("ghost123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    //change username
    user1.username =  String::from("hashghost123");

    let user2 = build_user(
        String::from("kyle@gmail.com"), 
        String::from("Kyle_hero")
    );

    let user3 = User{
        email: String::from("dee@gmail.com"), 
        username: String::from("busydee"),
        ..user2
    };

    println!("{}",user1.username);

}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}