fn main() {
    // loop
    // let x = loop {
    //     break 4;
    // };
    // println!("x = {x}");

    // while
    // let mut x = 0;
    // while x < 15 {
    //     x = x + 1;
    // }
    // println!("x = {x}");

    // for
    // let a = [10, 20, 30, 40, 50];
    // for el in a {
    //     println!("arr[*] = {}", el);
    // }

    let mut x = 0;
    'name_1: loop {
        x += 1;
        println!("name_1");
        'name_2: loop {
            println!("name_2");
            'name_3: while true {
                println!("name_3");
                if x == 3 {
                    break 'name_1;
                }
                continue 'name_1;
            }
        }
    }
}
