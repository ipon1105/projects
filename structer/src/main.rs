#[derive(Debug)]
struct Ractangle {
    width: f32,
    height: f32,
}

fn main() {
    let mut rect: Ractangle = Ractangle{
        width:33.3,
        height:23.9,
    };
    dbg!(rect.width = 34.0);
    let area: f32 = area(&rect);
    
    dbg!("rect = {:#?}", rect);
}

fn area(rect: &Ractangle) -> f32 {
    rect.width * rect.height
}