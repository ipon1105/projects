enum rub {
    one,
    two, 
    five,
}

fn main() {
    // match
    let money = rub::five;
    match money {
        rub::one => { println!("one"); },
        rub::two => println!("two"),
        rub::five => println!("five"),
    }

    // Шаблоны со значениями
    // Универсальный шаблон и заполнить _
    let x = 7;
    match x {
        5 => println!("Ты победил"),
        10 => println!("Повтори ещё"),
        other => println!("Выпало {other}. Ты проиграл"),
    }
    
}
