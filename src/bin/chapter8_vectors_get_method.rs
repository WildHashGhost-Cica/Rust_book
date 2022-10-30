fn main(){
    let mut v= vec![1,2,3];

    v.push(6); //we can't add extra element after when we borred 

    let third = &v[2];
    
    println!("The third element is {}", third);
    
    match v.get(2){
        Some(third)=> println!("This element is {}", third),
        None => println!("There is no third element"),
    }
}