#[derive(Debug)]
enum Direction {
    Right,
    Down,
    DiagonalDownLeft,
    DiagonalDownRight,
}

const XMAS: [&str; 2] = ["XMAS", "SAMX"];
const X_MAS: [&str; 2] = ["MAS", "SAM"];

fn can_move_dir(dir: &Direction, x_pos: usize, y_pos: usize, max_x: usize, max_y: usize) -> bool {
    match dir {
        Direction::Right => (x_pos + 3) <= max_x,
        Direction::Down => (y_pos + 3) <= max_y,
        Direction::DiagonalDownLeft => x_pos >= 3 && (y_pos + 3 <= max_y),
        Direction::DiagonalDownRight => (x_pos + 3 <= max_x) && (y_pos + 3 <= max_y),
    }
}

fn can_form_x(
    x_pos: usize,
    y_pos: usize,
    max_x: usize,
    max_y: usize,
    puzzle: &[Vec<char>],
) -> bool {
    puzzle[y_pos][x_pos] == 'A'
        && (y_pos >= 1)
        && (x_pos >= 1)
        && (y_pos < max_y)
        && (x_pos < max_x)
}

fn scan_in_dir(dir: &Direction, x_pos: usize, y_pos: usize, puzzle: &[Vec<char>]) -> String {
    let mut ret = String::with_capacity(4);
    for n in 0..=3 {
        match dir {
            Direction::Right => ret.push(puzzle[y_pos][x_pos + n]),
            Direction::Down => ret.push(puzzle[y_pos + n][x_pos]),
            Direction::DiagonalDownRight => ret.push(puzzle[y_pos + n][x_pos + n]),
            Direction::DiagonalDownLeft => ret.push(puzzle[y_pos + n][x_pos - n]),
        }
    }
    ret
}

fn is_x_mas(x_pos: isize, y_pos: isize, puzzle: &[Vec<char>]) -> bool {
    let mut arm_1 = String::with_capacity(3);
    let mut arm_2 = String::with_capacity(3);
    for n in (-1)..=1 {
        arm_1.push(puzzle[(y_pos + n) as usize][(x_pos + n) as usize]);
        arm_2.push(puzzle[(y_pos + n) as usize][(x_pos - n) as usize]);
    }
    X_MAS.contains(&arm_1.as_str()) && X_MAS.contains(&arm_2.as_str())
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|ln| ln.chars().collect()).collect()
}

pub fn solve_pt1(input: &str) -> usize {
    let puzzle = parse(input);
    let max_x = puzzle[0].len() - 1;
    let max_y = puzzle.len() - 1;
    let mut acc = 0;
    for j in 0..=max_y {
        for i in 0..=max_x {
            for dir in [
                Direction::Down,
                Direction::Right,
                Direction::DiagonalDownLeft,
                Direction::DiagonalDownRight,
            ] {
                if can_move_dir(&dir, i, j, max_x, max_y) {
                    let scan = scan_in_dir(&dir, i, j, &puzzle);
                    if XMAS.contains(&scan.as_str()) {
                        acc += 1
                    }
                }
            }
        }
    }
    acc
}

pub fn solve_pt2(input: &str) -> usize {
    let puzzle = parse(input);
    let max_x = puzzle[0].len() - 1;
    let max_y = puzzle.len() - 1;
    let mut acc = 0;
    for j in 0..=max_y {
        for i in 0..=max_x {
            if can_form_x(i, j, max_x, max_y, &puzzle) && is_x_mas(i as isize, j as isize, &puzzle)
            {
                acc += 1
            }
        }
    }
    acc
}

#[cfg(test)]

mod tests {
    use super::*;
    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 18)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 9)
    }
}
