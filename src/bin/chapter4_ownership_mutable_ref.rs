fn main(){
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}",s1);

    /*let mut s = String::from("rust");
    let r1 = &mut s;
    let r2 = &mut s; //it will throw an error, it can be borrow more than once
    //to fix we remove mut from s
    */
    let s = String::from("rust");
    let r1 = &s;
    let r2 = &s;
    
    println!("{}, {}", r1, r2);

    /*
    let mut n = String::from("test");
    let t1 = &n;
    let t2 = &n;
    let t3 = &mut n;
    cannot borrow as mutable if it was borrowed as immutable
    */

    let mut n = String::from("test");
    let t1 = &n;
    let t2 = &n;
    println!("{}, {}", t1, t2);

    let t3 = &mut n;
    println!("{}",t3);
}
fn change(some_string: &mut String){
    some_string.push_str(" world");
}