fn main() {
    let mut str = String::from("Hello world");
    let len = get_first_word(&str);

    // str.clear();
    println!("len of str({str}) = {len}");

    let a = [1,2,3,4,5];
    let slice = &a[2..5]; // Тут моя Ошибка была. Срез [2..4] - это значения 3, 4.
}

fn get_first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &iter) in bytes.iter().enumerate() {
        if iter == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}