use colored::*;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Heading<T: fmt::Display>(pub T);

impl<T: fmt::Display> fmt::Display for Heading<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_heading = self.0.to_string().bold().underline();
        write!(f, "{}", formatted_heading)
    }
}

impl<T: fmt::Display> From<T> for Heading<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
