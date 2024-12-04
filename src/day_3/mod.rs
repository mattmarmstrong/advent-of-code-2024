use itertools::Itertools;
use regex::Regex;

const MUL_SPLIT: &str = ",";

fn exec_mul_inst(inst: &str) -> usize {
    let ints = &inst[4..inst.len() - 1];
    let (x, y) = ints
        .split(MUL_SPLIT)
        .map(|x| x.parse::<usize>().ok().unwrap())
        .collect_tuple()
        .unwrap();
    x * y
}

fn exec_mul_inst_2(seq: Vec<&str>) -> usize {
    let mut should_exec: bool = true;
    let mut acc: usize = 0;
    for x in seq.iter() {
        match *x {
            "don't()" => {
                should_exec = false;
            }
            "do()" => {
                should_exec = true;
            }
            _ => {
                if should_exec {
                    acc += exec_mul_inst(x);
                } else {
                    continue;
                }
            }
        }
    }
    acc
}

pub fn solve_pt1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)").unwrap();
    re.find_iter(input).map(|x| exec_mul_inst(x.as_str())).sum()
}

pub fn solve_pt2(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+\))|do\(\)|don't\(\)").unwrap();
    let seq = re.find_iter(input).map(|s| s.as_str()).collect();
    exec_mul_inst_2(seq)
}

#[cfg(test)]

mod tests {
    use super::*;
    const EXAMPLE_PT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_PT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE_PT1), 161)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE_PT2), 48)
    }
}
