use crate::days::day2::people::People::{Draw, Me, Oppo};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum People {
    Me,
    Oppo,
    Draw,
}

impl Clone for People {
    fn clone(&self) -> Self {
        match self {
            Me => Me,
            Oppo => Oppo,
            Draw => Draw,
        }
    }
}

impl Copy for People {}

impl TryFrom<String> for People {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "X" => Ok(Oppo),
            "Y" => Ok(Draw),
            "Z" => Ok(Me),
            _ => Err(()),
        }
    }
}