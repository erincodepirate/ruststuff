fn main() {
    let greeting = "Hello";
    let subject = "World!";
    let greet = format!("Hello, {}!", subject);
    println!("{}", greet);
    println!("{}, {}!", greeting, subject);
    panic!("I crashed {}", greeting);
}