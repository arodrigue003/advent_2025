use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub struct PathCount(pub i64, pub i64, pub i64);

impl PathCount {
    pub fn new() -> Self {
        Self(0, 0, 0)
    }

    pub fn shift(&mut self) {
        self.2 += self.1;
        self.1 = self.0;
        self.0 = 0;
    }
}

impl Add for PathCount {
    type Output = PathCount;

    fn add(self, rhs: Self) -> Self::Output {
        PathCount(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for PathCount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
