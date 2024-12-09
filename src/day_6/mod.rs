use hashbrown::HashSet;
const GUARD: [char; 4] = ['<', '>', '^', 'v'];

fn parse_map(input: &str) -> ((isize, isize), Vec<Vec<char>>) {
    let mut guard_start: (isize, isize) = (0, 0);
    let map = input
        .lines()
        .take_while(|ln| !ln.is_empty())
        .enumerate()
        .map(|(y, ln)| {
            ln.chars()
                .enumerate()
                .map(|(x, c)| {
                    if GUARD.contains(&c) {
                        guard_start = (x as isize, y as isize)
                    }
                    c
                })
                .collect()
        })
        .collect();
    (guard_start, map)
}

#[inline]
fn guard_step(c: char) -> (isize, isize) {
    match c {
        '<' => (-1, 0),
        '>' => (1, 0),
        'v' => (0, 1),
        '^' => (0, -1),
        _ => panic!("Invalid Guard Character: {}", c),
    }
}

#[inline]
fn change_dir(c: char) -> char {
    match c {
        '<' => '^',
        '>' => 'v',
        'v' => '<',
        '^' => '>',
        _ => panic!("Invalid Guard Character: {}", c),
    }
}

fn get_step(
    x: isize,
    y: isize,
    guard: char,
    max_x: isize,
    max_y: isize,
    map: &mut Vec<Vec<char>>,
) -> Option<(isize, isize, char)> {
    let (step_x, step_y) = guard_step(guard);
    let new_x = x + step_x;
    let new_y = y + step_y;
    if new_x == -1 || new_y == -1 || new_x == max_x || new_y == max_y {
        return None;
    }
    match map[new_y as usize][new_x as usize] != '#' {
        true => Some((new_x, new_y, guard)),
        false => {
            let new_guard = change_dir(guard);
            get_step(x, y, new_guard, max_x, max_y, map)
        }
    }
}

fn set_distinct(start: (isize, isize), map: &mut Vec<Vec<char>>) -> HashSet<(isize, isize)> {
    let mut walked: HashSet<(isize, isize)> = HashSet::new();
    walked.insert(start);
    let (mut x, mut y) = start;
    let mut guard = map[y as usize][x as usize];
    map[y as usize][x as usize] = '.';
    let max_y = (map.len()) as isize;
    let max_x = (map[0].len()) as isize;
    while let Some((new_x, new_y, new_guard)) = get_step(x, y, guard, max_x, max_y, map) {
        x = new_x;
        y = new_y;
        guard = new_guard;
        walked.insert((x, y));
    }
    walked
}

fn is_in_loop(start: (isize, isize), map: &mut Vec<Vec<char>>) -> bool {
    let mut walked: HashSet<(isize, isize, char)> = HashSet::new();
    let (mut x, mut y) = start;
    let mut guard = map[y as usize][x as usize];
    walked.insert((x, y, guard));
    map[y as usize][x as usize] = '.';
    let max_y = (map.len()) as isize;
    let max_x = (map[0].len()) as isize;
    while let Some((new_x, new_y, new_guard)) = get_step(x, y, guard, max_x, max_y, map) {
        x = new_x;
        y = new_y;
        guard = new_guard;
        if walked.contains(&(x, y, guard)) {
            return true;
        }
    }
    false
}

pub fn solve_pt1(input: &str) -> usize {
    let (start, mut map) = parse_map(input);
    set_distinct(start, &mut map).len()
}

pub fn solve_pt2(input: &str) -> usize {
    let ((x, y), mut map) = parse_map(input);
    let mut acc = 0;
    let start_guard = map[y as usize][x as usize];

    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if map[j][i] != '#' && !(i == x as usize && j == y as usize) {
                map[j][i] = '#';
                if is_in_loop((x, y), &mut map) {
                    acc += 1;
                }
                map[y as usize][x as usize] = start_guard;
            }
        }
    }
    acc
}

#[cfg(test)]

mod tests {
    use super::*;
    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 41)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 6)
    }
}
