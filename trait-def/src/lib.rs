pub trait Test {
    type Exec;
}

pub struct First;
pub struct FirstArgument;

impl Test for First {
    type Exec = FirstArgument;
}
