use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Grid {
    pub start: (usize, usize),
    pub grid: Vec<Vec<char>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for block in line {
                write!(f, "{}", block)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
