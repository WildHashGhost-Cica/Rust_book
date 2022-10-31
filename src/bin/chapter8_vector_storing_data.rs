fn main(){
    enum SpreadsheeCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheeCell::Int(3),
        SpreadsheeCell::Text(String::from("blue")),
        SpreadsheeCell::Float(10.12),
    ];
    match &row[1]{
        SpreadsheeCell::Int(i) => println!("{}",i),
        _ => println!("Not an integer!")
    };
}