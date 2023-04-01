fn main() {
    // Строки - Изменение
    let x: String = String::new();
    let x: String = "string".to_string();
    let mut x: String = String::from("strings");
    println!("x = {}", x);

    // x.push('a');
    x.push_str(" is cool");
    println!("x = {}", x);

    let a: String = String::from("He你llo");
    let space: String = String::from(" ");
    let b: String = String::from("World");
    
    // let res = a + &space + &b;
    let res = format!("{a} {b}");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("res = {}", res);
    // Индексирование
    // let x = res[0];
    // Байты, Скалярные значения и Кластеры Графем
    // Срезы строк
    // let s = &res[0..4];
    // println!("s = {}", s);
    // Перебор
    for c in "3Д".chars(){
        println!("c = {c}");
    }
    for c in "3 Д".bytes(){
        println!("byte = {c}");
    }
}