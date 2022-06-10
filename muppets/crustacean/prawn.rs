use crate::Says;

// THIS METHOD, Pepe has no interest in being imported into Sesame Street.
pub(in crate::muppets) struct Pepe {}

// If you change to this, Pepe can be used in Sesame Street.
//pub struct Pepe {}

impl Says for Pepe {
    fn say(&self) -> String {
        "What you talking about".to_owned()
    }
}
