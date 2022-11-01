use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(70); //this line won't do nothing 
    

    println!("{:?}", scores);
}
