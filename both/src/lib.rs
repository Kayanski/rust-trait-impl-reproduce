pub trait Test {
    type Exec;
}
pub struct First;
pub struct FirstArgument;

impl Test for First {
    type Exec = FirstArgument;
}
pub struct Second;

pub struct SecondArgument;

impl Test for Second {
    type Exec = SecondArgument;
}

impl From<<First as Test>::Exec> for SecondArgument {
    fn from(value: <First as Test>::Exec) -> Self {
        Self
    }
}
