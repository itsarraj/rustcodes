struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 10,
        height: 20,
    };
    let x = area(&rec1);
    println!("{}", x);
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
