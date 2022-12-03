use crate::days::day2::shape::Shape::{A, B, C};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum Shape {
    A,
    B,
    C,
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            A => "A".to_string(),
            B => "B".to_string(),
            C => "C".to_string(),
        }
    }
}

impl TryFrom<String> for Shape {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "A" => Ok(A),
            "B" => Ok(B),
            "C" => Ok(C),
            _ => Err(()),
        }
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        match self {
            A => A,
            B => B,
            C => C,
        }
    }
}

impl Copy for Shape {}