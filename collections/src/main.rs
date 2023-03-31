#[derive(Debug)]
enum SpredsheetCell {
    Int(i32),
    Float(f64),
}

fn main() {
    // Коллекция Vec<T>
    // Создание, Изменение, Чтение
    let v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![0; 5];
    // let mut v: Vec<i32> = vec!(1,2,3,4);
    let mut v: Vec<SpredsheetCell> = Vec::new();

    v.push(SpredsheetCell::Int(3));
    v.push(SpredsheetCell::Float(3.5));

    let x: &mut SpredsheetCell = &mut v[0];
    *x = SpredsheetCell::Int(4);

    // println!("x = {}", v[100]);
    // println!("x = {}", v.get(100));
    
    // v[0] = 1000;

    // Перебор
    for i in &v {
        println!("i = {:?}", i);
    }
    for i in &mut v {
        println!("i = {:?}", i);
    }

}
