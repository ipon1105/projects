fn main() {
    // Скалярные типы данных
    // Целочисленные
    // i8, i16, i32, i64, i128 (-128 127) 
    // u8, u16, u32, u64, u128 (0 255)
    // isize, usize
    let a: i8 = 10;
    let b: i8 = 15;

    // Вещественные
    // f32, f64
    let c1: f32 = 3.0;
    let c2: f32 = 5.7;

    // Логический
    let t: bool = true;
    let f: bool = false;

    // Символьный
    let c = 'A';

    // Сложные типы данных
    // Кортежи
    let tup: (i8, i8) = (10, 15);
    println!("tup = {}", tup.1);

    let (a, b) = tup;
    println!("a = {}", a);
    println!("b = {}", b);

    // Массивы
    let arr = [1,2,3,4,5];
    let arr = [0; 5];
    let arr: [u8; 5];

    println!("arr[0] = {}", arr[0]);
    
}
