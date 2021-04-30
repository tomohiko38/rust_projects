use std::fs::File;

fn main() {

    // 自らパニックを起こす
    //panic!("crash and burn");
    
    // out of range の例
    //let v = vec![1, 2, 3];
    //v[99];
    
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("The was a problem opening the file: {:?}", error);
        },
    };
    
    
}
