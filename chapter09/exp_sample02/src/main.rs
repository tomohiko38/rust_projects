use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello2.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // NotFound だったらここでファイルを作成する
            match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("The was a problem opening the file: {:?}", error)
        },
    };
}