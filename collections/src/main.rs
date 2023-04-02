fn main() {
    // HashMap
    use std::collections::HashMap;

    // Создание, Изменение, Владение, Обновление
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Red"), 21);

    let str = "Bluu".to_string();
    let x = scores.get(&str).copied().unwrap_or(0);
    println!("x = {x}");
    
    for (key, value) in &scores {
        println!("key({key}) = value({value})");
    }

    let xxx = scores.entry(String::from("Red")).or_insert(20);
    for (key, value) in &scores {
        println!("key({key}) = value({value})");
    }
    
    let text = "hello world wonderful world";
    let mut word_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_map);
}