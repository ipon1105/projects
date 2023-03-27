#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

fn main() {
    let mut rect: Rectangle = Rectangle{
        width:33.3,
        height:23.9,
    };
    dbg!(rect.width = 34.0);
    let area: f32 = area(&rect);
    
    dbg!("rect = {:#?}", rect);
}

fn area(rect: &Rectangle) -> f32 {
    rect.width * rect.height
}