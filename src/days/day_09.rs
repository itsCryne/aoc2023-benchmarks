pub fn part_a(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .map(|report: Vec<i64>| {
                std::iter::successors(Some(report), |last| {
                    if last.iter().sum::<i64>() == 0 {
                        None
                    } else {
                        Some(last.windows(2).map(|w| w[1] - w[0]).collect())
                    }
                })
                .map(|h| *h.last().unwrap())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .reduce(|acc, e| acc + e)
                .unwrap()
            })
            .sum(),
    )
}

pub fn part_b(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .map(|report: Vec<i64>| {
                std::iter::successors(Some(report), |last| {
                    if last.iter().sum::<i64>() == 0 {
                        None
                    } else {
                        Some(last.windows(2).map(|w| w[1] - w[0]).collect())
                    }
                })
                .map(|h| *h.first().unwrap())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .reduce(|acc, e| e - acc)
                .unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day_009_part_a() {
        let input_a = read_to_string("./data/examples/day_09_a.txt").unwrap();
        let result = part_a(input_a.as_str());
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_day_009_part_b() {
        let input_b = read_to_string("./data/examples/day_09_b.txt").unwrap();
        let result = part_b(input_b.as_str());
        assert_eq!(result, Some(2));
    }
}
