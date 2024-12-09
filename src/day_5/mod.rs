use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

const DOUBLE_NEWLINE: &str = "\n\n";
const SEPARATOR: &str = "|";
const COMMA: &str = ",";

type OrderMap = HashMap<usize, HashSet<usize>>;
type Updates = Vec<Vec<usize>>;

fn parse(input: &str) -> (OrderMap, Updates) {
    let (order, updates): (&str, &str) = input.split(DOUBLE_NEWLINE).collect_tuple().unwrap();
    let mut order_map: OrderMap = HashMap::new();
    order.lines().for_each(|ln| {
        let (first, second): (usize, usize) = ln
            .split(SEPARATOR)
            .map(|x| x.parse::<usize>().ok().unwrap())
            .collect_tuple()
            .unwrap();
        if order_map.contains_key(&first) {
            let v = order_map.get_mut(&first).unwrap();
            v.insert(second);
        } else {
            let mut v = HashSet::new();
            v.insert(second);
            order_map.insert(first, v);
        }
    });
    let updates = updates
        .lines()
        .map(|ln| {
            ln.split(COMMA)
                .take_while(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().ok().unwrap())
                .collect()
        })
        .collect();
    (order_map, updates)
}

fn check_if_ord(order_map: &OrderMap, update_ord: &Vec<usize>) -> bool {
    let l = update_ord.len();
    if update_ord.is_empty() || l == 1 {
        return true;
    }
    update_ord.iter().enumerate().all(|(i, pg)| {
        if i < l {
            update_ord[0..i].iter().all(|x| {
                if order_map.contains_key(pg) {
                    let befores = order_map.get(pg).unwrap();
                    !befores.contains(x)
                } else {
                    true
                }
            })
        } else {
            true
        }
    })
}

#[inline]
fn get_median(v: &[usize]) -> usize {
    let m = ((v.len() / 2) as f64).ceil() as usize;
    v[m]
}

fn get_next(order_map: &OrderMap, un_ord: &Vec<usize>) -> Vec<usize> {
    let first = *un_ord.first().unwrap();
    let mut tmp = Vec::with_capacity(un_ord.len());
    un_ord.iter().for_each(|n| {
        if order_map.contains_key(n) {
            let befores = order_map.get(n).unwrap();
            if befores.contains(&first) {
                tmp.push(*n);
            }
        }
    });
    if tmp.is_empty() {
        vec![first]
    } else if check_if_ord(order_map, &tmp) {
        tmp
    } else {
        get_next(order_map, &tmp)
    }
}

fn order(order_map: &OrderMap, un_ord: &mut Vec<usize>) -> Vec<usize> {
    let mut tmp = Vec::with_capacity(un_ord.len());
    while !un_ord.is_empty() {
        let ord = get_next(order_map, un_ord);
        tmp.append(&mut ord.clone());
        let new_vec = un_ord
            .iter()
            .filter(|x| !(ord.contains(x)))
            .copied()
            .collect::<Vec<usize>>();
        let _ = std::mem::replace(un_ord, new_vec);
        if check_if_ord(order_map, un_ord) {
            tmp.append(un_ord);
            break;
        }
    }
    tmp
}

pub fn solve_pt1(input: &str) -> usize {
    let (ord_map, updates) = parse(input);
    updates
        .iter()
        .map(|u| {
            if check_if_ord(&ord_map, u) {
                get_median(u)
            } else {
                0
            }
        })
        .sum()
}

pub fn solve_pt2(input: &str) -> usize {
    let (ord_map, mut updates) = parse(input);
    updates
        .iter_mut()
        .filter(|u| !check_if_ord(&ord_map, u))
        .map(|u| get_median(order(&ord_map, u).as_slice()))
        .sum()
}

#[cfg(test)]

mod tests {
    use super::*;
    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 143)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 123)
    }
}
