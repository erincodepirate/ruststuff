fn main() {
    let years: [i32; 3] = [1995, 2000, 2005];
    for year in years.iter() {
        println!("Next year: {}", year + 1);
    }
}