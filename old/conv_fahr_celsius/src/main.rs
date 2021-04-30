fn main() {

    let mut celsius = 0.0;
    let step = 10.0;
    let max_celsius = 100.0;

    while celsius < max_celsius {
        let fahr = conv_c_to_f(celsius);
        println!("celsius:{} fahr:{}", celsius, fahr);
        celsius = celsius + step;
    }
}

fn conv_c_to_f(celsius: f32) -> f32 {
    celsius / (5.0 / 9.0) + 32.0
}