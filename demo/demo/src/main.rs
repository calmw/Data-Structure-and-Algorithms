use std::fs::File;
use std::io;
use std::io::{Error, Read, read_to_string};

fn main() {
    let s = read_from_file();
    match s {
        Ok(e) => println!("{}", e),
        Ok(s) => println!("{}", s),
        _ => {}
    }
}

fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("./key.txt")?; // 出错时直接抛出
    let mut s = String::new();
    f.read_to_string(&mut s).expect("TODO: panic message");
    Ok(s)
}
