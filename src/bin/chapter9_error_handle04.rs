use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f = File::open("hello1.text").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello1.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file {:?}",error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}