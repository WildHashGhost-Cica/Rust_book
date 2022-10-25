fn main(){
    let mut s = String::from("hello wildhashghost");
    let s2 = "hello wildhashghost";

    let word = first_word(s2);

    let a = [1,2,3,4,5];
    let slice =  &a[0..2];
    println!("{:?}", slice);
    println!("{:?}", word);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}