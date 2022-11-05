use std::fs::File;


fn main(){
    /*enum Result<T, E>{
        Ok(T),
        Err(E).
    }*/

    let f = File::open("hello.text");

    let f = match f{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}