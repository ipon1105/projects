pub enum type_of_cost {
    free,
    expansive,
}

#[derive(Debug)]
pub struct Tomat{
    count: u8,
    pub type_of_plant: String,
}

impl Tomat {
    pub fn plant(type_of_plant: String) -> Self {
        Self {
            count: 3,
            type_of_plant,
        }
    }
}

pub fn from_vegetables(){
    println!("hello from vegetables");
}

pub fn call_garden(){
    super::from_garden();
}