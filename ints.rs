fn multiply(x: i64, y:u8) -> i64 {
    return x * (y as i64);
}

fn divide(x: i32, y: u16) -> f64 {
    return x as f64 / y as f64;
}

fn main() {
    let ninety = 90;
    let negative_five = -5;
    let one_thousand = 1_000;
    let exactly_three = 10 / 3;
    println!("{} {} {} {}", ninety, negative_five, one_thousand, exactly_three);
    
    let x = multiply(4, 2);
    let y = divide(4, 2);

    println!("{} {}", x, y)
}