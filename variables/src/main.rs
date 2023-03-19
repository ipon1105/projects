const PI: f64 = 3.1415;

fn main() {
    let str = "1000";
    println!("str before = {str}");

    {
        let str = 9999;
        println!("your number is {str}");
    }

    println!("str after = {str}");
    println!("PI is {PI}");

}
