use std::error::Error;
use std::fmt::{Display, Formatter, Result as Res_f};

#[derive(Debug)]
pub struct Err(Box<dyn Error>);

impl Display for Err {
    fn fmt(&self, f: &mut Formatter) -> Res_f {
        self.0.fmt(f)
    }
}

impl<E: Error + 'static> From<E> for Err {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}
