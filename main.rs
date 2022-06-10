mod muppets;
mod sesame_street;
use muppets::*;

pub trait Says {
    fn say(&self) -> String {
        "Hello".to_owned()
    }
}

fn main() {
    let de_frog = Kermit {};
    println!("{}", de_frog.say());
}
