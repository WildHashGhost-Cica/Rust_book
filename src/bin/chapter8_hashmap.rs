use std::collections::HashMap;

fn main(){
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut score = HashMap::new();

    score.insert(&blue, 10);
    score.insert(&yellow, 50);

    println!("{}", blue);
    let team_name = String::from("Blue");
    let score = score.get(&team_name);

    
    
}