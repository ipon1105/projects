fn main() {
    // Ссылки и заимствование
    let mut s = String::from("hello");
    let l = get_len(&s);

    println!("{s} len = {l}");

    // Изменяемые ссылки
    set_world(&mut s);
    let l = get_len(&s);
    println!("{s} len = {l}");

    let a = &s;
    let b = &s;
    println!("{a} {b}");


    let c = &mut s;
    println!("{c}");

    // Висячие ссылки
    let d = get_str();
}

fn get_str() -> &String{
    let str = String::from("stroka");
    &str
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn set_world(s: &mut String) {
    s.push_str(" world");
}