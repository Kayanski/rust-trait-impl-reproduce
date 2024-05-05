use trait_def::Test;

pub struct First;
pub struct FirstArgument;

impl Test for First {
    type Exec = FirstArgument;
}
