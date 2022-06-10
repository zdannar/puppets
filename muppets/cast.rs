use crate::{muppets::Pepe, Kermit, Says};
pub struct Cast {
    cast: Vec<Box<dyn Says>>,
}

impl Cast {
    pub fn new() -> Self {
        Self {
            cast: vec![Box::new(Kermit {}), Box::new(Pepe {})],
        }
    }
}
