use unicode_segmentation::UnicodeSegmentation;

fn main(){
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);

    let s5 = s1 + s2;
    println!("{}", s5);

    let hello =String::from("Hello");
   
    /*let c: char = hello[0];
    println!("{}", c);*/
    for b in "Hello".bytes(){
        println!("{}",b);
    }
    //scalar values
    for c in "Hello".chars(){
        println!("{}",c);
    }
    
    //added to Cargo.toml some dependencies : unicode-segmentation="*"
    //Grapheme clusters
    for g in "WildhashGhost".graphemes(true){
        println!("{}",g);
    }

}