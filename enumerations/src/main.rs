enum coin {
    one,
    two,
    five,
    ten(u8),
}

fn main() {
    // if let

    let money = coin::ten(3);
    if let coin::ten(v) = money {
        println!("ten have a {v}. you get a candy");
    } else {
        println!("you not have a money");
    }
}
