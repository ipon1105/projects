
fn a<F: Fn()>(f: F){
    f();
    f();
}

fn main() {
    // fn add_one_v1(x: u32) -> u32 { x + 1 }
    let mut my_x: u32 = 0;
    let mut my_x2: i32 = 0;
    let add_one_v2 = | x: u32 | -> u32 { x + 1 };
    let add_one_v3 = | x | { x + 1 };
    let add_one_v4 = | x | x + 1;

    println!("my_x = {my_x}");

    my_x = add_one_v3(my_x);
    println!("my_x = {my_x}");
    // my_x = add_one_v3(my_x2);

    my_x = add_one_v4(my_x);
    println!("my_x = {my_x}");

    // ---------------------------------------- //

    let example_closure = || println!("{my_x}");
    example_closure();

    // ---------------------------------------- //

    let list = vec![1, 2, 3];
    println!("vec = {:?}", list);

    let only_borrows = || println!("form closures vec = {:?}", list);

    println!("before call only_borrows vec = {:?}", list);
    only_borrows();
    println!("after call only_borrows vec = {:?}", list);

    // ---------------------------------------- //

    let mut list = vec![1, 2, 3];
    println!("vec = {:?}", list);

    let mut muttable_borrows = || println!("form closures vec = {:?}", list);

    // println!("before call only_borrows vec = {:?}", list);
    only_borrows();
    println!("after call only_borrows vec = {:?}", list);

    // ---------------------------------------- //

    let list = vec![1, 2, 3];
    println!("vec = {:?}", list);

    let muttable_borrows = move || println!("form closures vec = {:?}", list);

    // println!("before call only_borrows vec = {:?}", list);
    only_borrows();
    // println!("after call only_borrows vec = {:?}", list);

    // FnOnce
    // FnMut
    // Fn

    a(|| println!("Hi"));
    // a(|| println!("Hi"));
}
