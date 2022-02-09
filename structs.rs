struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn new_point(x: i64, y: i64, z:i64) -> Point {
    return Point {x, y, z}
}

fn main() {
    let n = new_point(1,2,3);
    println!("{} {} {}", n.x, n.y, n.z);
    let p = Point {x:1, y:2, z:3};
    println!("{} {} {}", p.x, p.y, p.z);
}