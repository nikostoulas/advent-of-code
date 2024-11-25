pub struct SolutionDay {
    pub year: u32,
    pub day: u8,
    pub part: u8,
}

impl SolutionDay {
    pub fn create(year: u32, day: u8, part: u8) -> Self {
        Self { year, day, part }
    }
}
