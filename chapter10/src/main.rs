#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}
struct Point_f64_char{
    x: f64,
    y: char,
}
struct Point_char_char{
    x: char,
    y: char,
}


impl<X, Y> Point<X, Y> {
    // fn x(&self) -> &X {
    //     &self.x
    // }

    // fn y(&self) -> &Y {
    //     &self.y
    // }

    fn mixup<X2, Y2> (self, other: Point<X2, Y2>) -> Point<X, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let x = Point{x: 3.0, y: 'c'};
    let y = Point{x: 'a', y: 'd'};

    let z = y.mixup(x);
    println!("z = {:?}", z);

    // println!("x = {}", x.x());
    // println!("y = {}", x.y());

    // let v = [1,2,3,4,5,6,7];
    // let max = largest(&v);
    // println!("{max}");

    // let v = [1.0,2.0,3.0,4.0,5.0,6.0,8.0];
    // let max = largest(&v);
    // println!("{max}");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if largest < i {
            largest = i;
        }
    }
    largest
}