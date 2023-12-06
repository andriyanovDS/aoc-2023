use anyhow::Result;

pub trait DaySolution {
    fn first_part() -> Result<String>;
    fn second_part() -> Result<String>;
}
