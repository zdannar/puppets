use crate::Says;
pub struct Oscar {}

impl Says for Oscar {
    fn say(&self) -> String {
        "I love trash!".to_owned()
    }
}
