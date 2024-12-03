#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    todo!("day x - part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("tests - part1");
        let input = "";
        assert_eq!(process(input)?, "");
        Ok(())
    }
}
