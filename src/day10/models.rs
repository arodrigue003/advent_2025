use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Vec<i8>>,
    pub joltages: Vec<i64>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "lights: [")?;
        for light in &self.lights {
            if *light {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "], buttons: ")?;
        for button in &self.buttons {
            write!(f, "(")?;
            for line in button {
                write!(f, "{line},")?;
            }
            write!(f, ") ")?;
        }
        write!(f, "joltages: {{")?;
        for joltage in &self.joltages {
            write!(f, "{joltage},")?;
        }
        write!(f, "}}")
    }
}
