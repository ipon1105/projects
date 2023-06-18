fn main() {
    let v1 = vec![1, 2, 3, 6];
    let iterator = v1.iter();
    // let iterator = v1.iter_mut();
    // let iterator = v1.into_iter();

    // for item in iterator {
    //     println!("item = {item}");
    // }
    // println!("{:?}", v1);

    // let iterator = v1.iter();
    // let total: i32 = iterator.sum();
    // println!("{total}");

    let x: Vec<_> = iterator.map( | x | x + 1 ).collect();
    println!("{:?}", x);
}
