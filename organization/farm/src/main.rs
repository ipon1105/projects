mod garden;

use garden::from_garden;
use garden::vegetables::from_vegetables;    
use garden::vegetables::call_garden;
use garden::vegetables::Tomat;
use garden::vegetables::type_of_cost;

fn main() {
    from_garden();
    from_vegetables();
    call_garden();

    let x = Tomat::plant(String::from("yellow"));
    println!("x = {:?}",x);
}
