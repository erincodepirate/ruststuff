fn main() {
    let point: (i64, i64, i64) = (1, 2, 3);

    let x = point.0;
    let y = point.1;
    let z = point.2;

    println!("{} {} {}", x, y, z);

    let (x,y,z) = point;

    println!("{} {} {}", x, y, z);

    let (x,y,_) = point;

    println!("{} {}", x, y);
}