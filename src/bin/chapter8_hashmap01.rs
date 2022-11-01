use std::collections::HashMap;

fn main(){
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    for (key, value) in &scores {
        println!("{} :{}", key, value);
    }
}