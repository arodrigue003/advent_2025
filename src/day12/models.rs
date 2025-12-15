#[derive(Debug, Clone)]
pub struct Shape {
    pub shape: Vec<Vec<bool>>,
}

impl Shape {
    pub fn pixel_count(&self) -> i64 {
        self.shape
            .iter()
            .flat_map(|line| line.iter().map(|value| if *value { 1 } else { 0 }))
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub size: (i64, i64),
    pub repetitions: Vec<i64>,
}