pub struct Dimacs;

impl Dimacs {
    pub fn parse(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
        input
            .lines()
            .map(str::trim_start)
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with('c'))
            .filter(|line| !line.starts_with('p'))
            .map(|line| -> anyhow::Result<_> {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().map_err(|e| anyhow::Error::from(e)))
                    .filter(|n| !matches!(n, Ok(0)))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let dimacs = "
        c test comment \n\
        p 3 3          \n\
        1 -2 -3 0      \n\
        2 3 1 0        \n\
        1 0            \n\
        2 0            
        ";

        let result = Dimacs::parse(dimacs).unwrap();
        assert_eq!(result[0], vec![1, -2, -3]);
        assert_eq!(result[1], vec![2, 3, 1]);
        assert_eq!(result[2], vec![1]);
        assert_eq!(result[3], vec![2]);
    }
}
