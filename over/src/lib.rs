use trait_def::Test;
use under::{First, FirstArgument};

pub struct Second;

pub enum SecondArgument {
    First(FirstArgument),
}

impl Test for Second {
    type Exec = SecondArgument;
}

// This doesn't work
impl From<<First as Test>::Exec> for SecondArgument {
    fn from(value: <First as Test>::Exec) -> Self {
        Self::First(value)
    }
}

// This would work
// impl From<FirstArgument> for SecondArgument {
//     fn from(value: <First as Test>::Exec) -> Self {
//         Self::First(value)
//     }
// }
