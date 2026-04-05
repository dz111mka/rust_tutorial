fn main() {
    let rectangle = Rectangle {
        width: dbg!(30 * 2),
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );

    dbg!(&rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}