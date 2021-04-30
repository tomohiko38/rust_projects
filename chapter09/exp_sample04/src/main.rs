use std::io;
use std::io::Read;
use std::fs::File;

fn main() {

    // Result<T,E>型の扱い(1)
    //let s = read_username_from_file().expect("失敗した");
    //println!("{}", s);
    
    // Result<T,E>型の扱い(2)
    let s = read_username_from_file();
    let s = match s {
        Ok(file) => file,
        Err(error) => {
            panic!("うわあああああ!: {:?}", error)
        },
    };
    println!("{}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}