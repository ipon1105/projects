#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(self: &Self, rect: &Rectangle) -> bool{
        self.height > rect.height && self.width > rect.width
    }
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Self { 
            width, 
            height,
        }
    }
}

fn main() {
    // Метод
    // Метод с несколькими параметрами
    // Ассоциированные функции

    let rect0: Rectangle = Rectangle{
        width:33.3,
        height:23.9,
    };
    let rect1: Rectangle = Rectangle::new(20.0, 13.9);
    

    let area: f32 = (&rect0).area();
    dbg!("rect0 = {:?}", &rect0);
    dbg!("rect1 = {:?}", &rect1);
    dbg!("area = {}", area);
    dbg!("rect0 can hold rect1 = {}", rect0.can_hold(&rect1));
    dbg!("rect1 can hold rect0 = {}", rect1.can_hold(&rect0));
}

