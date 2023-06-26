use crate::garden::vegetables::Potato;

pub mod garden;

fn main() {
    println!("This is about managing project.");

    garden::manage_garden();

    let plant = Potato {};
    println!("I'm growing {:?}!", plant);
}
