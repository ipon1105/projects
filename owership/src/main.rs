fn main() {
    // Владение и Функции
    // foo();

    // Владение и возращения значений
    // bar();
}

fn foo(){
    let str = String::from("hello");  // str создаётся :)

    takes_ownership(str);     // Значение str передаётся как аргумент функции
    // takes_ownership(str);     // (ОШИБКА) str больше не существует :( 
    

    let x = 5;              // x создаётся

    makes_copy(x);          // Значение x передаётся как аргумент функции
    makes_copy(x);          // Посколько x простой тип, то оно передаётся копией
                            // и x продолжает существовать
    println!("x = {x}");
}
// Вызвает drop для x и str однако str 
// Уже было уничтожено в после выполнения
// функции, так что ничего не делаем

fn takes_ownership(mut str: String) { // str создаётся
    str.push_str("+str");
    println!("{}", str);
}
// Вызывается drop - str уничтожается

fn makes_copy(mut i: i32) { // int создаётся
    i = i + 5;
    println!("{}", i);
} 
// Вызывается drop - int уничтожается

fn bar(){
    let s1 = String::from("hello");
    let s2 = add_world(s1);
    
    // println!("{s1}");
    println!("{s2}");
}
// drop вызываем только для s1, s2 уже уничтожен

fn add_world(mut str: String) -> String {
    str.push_str(" world");     // world добавляется
    str
}
// При возрате str мы передаём право владение другому владельцу
// Поэтому drop для str не вызывается


