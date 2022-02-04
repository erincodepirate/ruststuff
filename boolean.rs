fn main() {
    let cats = 2;
    if cats > 1 {
        println!("Multiple cats!");
    } else if cats > 1_000 {
        println!("Too many cats!");
    }
}