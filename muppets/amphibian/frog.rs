use crate::Says;

pub(crate) struct Kermit;

impl Says for Kermit {
    fn say(&self) -> String {
        return "It ain't easy being green.".to_owned();
    }
}
