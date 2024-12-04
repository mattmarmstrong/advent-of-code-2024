fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .take_while(|ln| !ln.is_empty())
        .map(|ln| {
            ln.split_whitespace()
                .map(|x| x.parse::<usize>().ok().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn all_dsc_asc(ln: &[usize]) -> bool {
    let mut ln_iter = ln.iter();
    let first = ln_iter.next().unwrap();
    let mut curr = ln_iter.next().unwrap();
    let asc: bool;

    if first < curr && (*first..=(*first + 3)).contains(curr) {
        asc = true;
    } else if first > curr && (first.checked_sub(3).unwrap_or(0)..=*first).contains(curr) {
        asc = false;
    } else {
        return false;
    }

    for x in ln_iter {
        if (asc && x > curr && (curr + 3 >= *x))
            || (!asc && x < curr && (curr.checked_sub(3).unwrap_or(0)) <= *x)
        {
            curr = x;
        } else {
            return false;
        }
    }
    true
}

fn all_dsc_asc_2(ln: &Vec<usize>) -> bool {
    match all_dsc_asc(ln) {
        true => true,
        false => {
            for i in 0..ln.len() {
                let mut ln2 = ln.clone();
                ln2.remove(i);
                if all_dsc_asc(&ln2) {
                    return true;
                }
            }
            false
        }
    }
}

pub fn solve_pt1(input: &str) -> usize {
    let lns = parse(input);
    lns.iter().filter(|x| all_dsc_asc(x)).count()
}

pub fn solve_pt2(input: &str) -> usize {
    let lns = parse(input);
    lns.iter().filter(|x| all_dsc_asc_2(x)).count()
}

#[cfg(test)]

mod tests {
    use super::*;
    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 2)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 4)
    }
}
