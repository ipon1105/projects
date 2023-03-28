#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
    V8{x: i32, y: i32},
}

impl IpAddrKind {
    fn print(&self) {
        println!("ip = {:?}", self);
    }
}

fn main() {
    // Перечисление (enumeration)
    // Значение перечислений
    // Методы для перечислений
    let ip4: IpAddrKind = IpAddrKind::V4(192, 168, 0, 1);
    let ip6: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    let ip8: IpAddrKind = IpAddrKind::V8{x:32, y:32};

    ip4.print();
    ip6.print();
    ip8.print();

    let x: Option<i32> = Some(32);
    let y: i32 = 15;
    
    // let z = x + y;
    // Перечисление Option<T> и почему в Rust нет null
    // Если Вы попытаетесь использовать null значение в качестве not-null значения,
    // Вы получите ошибку определённого рода
}
