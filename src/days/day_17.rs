pub fn part_a(_input: &str) -> Option<u32> {
    None
}

pub fn part_b(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day_17_part_a() {
        let input_a = read_to_string("./data/examples/day_17_a.txt").unwrap();
        let result = part_a(input_a.as_str());
        assert_eq!(result, None);
    }

    #[test]
    fn test_day_17_part_b() {
        let input_b = read_to_string("./data/examples/day_17_b.txt").unwrap();
        let result = part_b(input_b.as_str());
        assert_eq!(result, None);
    }
}
