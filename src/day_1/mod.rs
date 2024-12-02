use itertools::Itertools;

const NEWLINE: &str = "\n";

pub fn solve_pt1(input: &str) -> usize {
    let mut left_line = Vec::new();
    let mut right_line = Vec::new();
    input
        .split(NEWLINE)
        .take_while(|x| !x.is_empty())
        .for_each(|ln| {
            let (x, y) = ln
                .split_whitespace()
                .map(|x| x.parse::<usize>().ok().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap();

            left_line.push(x);
            right_line.push(y);
        });
    left_line.sort();
    right_line.sort();

    left_line
        .iter()
        .zip(right_line.iter())
        .map(|(x, y)| usize::abs_diff(*x, *y))
        .sum()
}

pub fn solve_pt2(input: &str) -> usize {
    let mut left_line = Vec::new();
    let mut right_line = Vec::new();
    input
        .split(NEWLINE)
        .take_while(|x| !x.is_empty())
        .for_each(|ln| {
            let (x, y) = ln
                .split_whitespace()
                .map(|x| x.parse::<u64>().ok().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap();

            left_line.push(x);
            right_line.push(y);
        });

    left_line
        .iter()
        .map(|x| {
            let sim_score = right_line.iter().filter(|y| x == *y).count();
            *x as usize * sim_score
        })
        .sum()
}

#[cfg(test)]

mod tests {
    use super::*;
    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 11)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 31)
    }
}
