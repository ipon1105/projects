struct User {
    email: String,
    password: String,
    age: i32,
}

struct Pixel(u8, u8, u8);
struct Color(u8, u8, u8);

struct StucterName;

fn main() {
    // Структура 
    let user = build_user(String::from("email1"), String::from("456"));

    // Синтаксис обновления структуры
    let user2 = User {
        email: String::from("email2"),
        ..user
    };

    // println!("{}, {}, {}", user.email, user.password, user.age);
    println!("{}, {}, {}", user2.email, user2.password, user2.age);

    // Кортежные структуры
    let pixel1 = Pixel(3,2,1);
    println!("{}, {}, {}", pixel1.0, pixel1.1, pixel1.2);

    // Единично-подобные структуры
    let x = StucterName;
    println!("{}", x);
}

fn build_user(email: String, password: String) -> User{
    User {
        email,
        password,
        age: 24
    }
}