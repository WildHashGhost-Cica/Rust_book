fn main(){
    let a= [1,2,3];
    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}",v);
    {
    let v2: Vec<i32> = vec![1,2,3];
    println!("v2 = {:?}", v2);
    }
    let third = &v[2];
    println!("The third element is {}", third);
}