use trait_def::Test;
use trait_def::{First, FirstArgument};

pub struct Second;

pub struct SecondArgument;

impl Test for Second {
    type Exec = SecondArgument;
}

// This doesn't work
impl From<<First as Test>::Exec> for SecondArgument {
    fn from(value: <First as Test>::Exec) -> Self {
        Self
    }
}

// This would work
// impl From<FirstArgument> for SecondArgument {
//     fn from(value: <First as Test>::Exec) -> Self {
//         Self::First(value)
//     }
// }
