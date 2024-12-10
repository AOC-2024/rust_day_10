use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

pub fn count_trail_heads_score(input_path: &str) -> u32 {
    let mut tail_heads_score = 0;
    let puzzle = read_to_string(input_path).unwrap();
    let starting_points = find_starting_points(&puzzle);

    let array: Vec<Vec<usize>> = to_two_dimensional_array(&puzzle);

    for point in starting_points.iter() {
        tail_heads_score += tail_head_from_starting_point_count(&array, point);
    }

    tail_heads_score
}

fn to_two_dimensional_array(puzzle: &str) -> Vec<Vec<usize>> {
    let mut rows = Vec::new();

    puzzle.lines().for_each(|line| {
        let mut colum = Vec::new();
        line.chars().for_each(|c| {
            colum.push(c.to_digit(10).unwrap() as usize);
        });
        rows.push(colum);
    });

    rows
}

fn find_starting_points(puzzle: &str) -> Vec<Point> {
    puzzle
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '0')
                .map(|(x, _)| Point(y, x))
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn tail_head_from_starting_point_count(array: &Vec<Vec<usize>>, point: &Point) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut score = 0;

    queue.push_back(point.clone());
    visited.insert(point.clone());

    while let Some(Point(x, y)) = queue.pop_front() {
        for (nx, ny) in [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, (y + 1)),
        ] {
            if let Some(row) = array.get(nx) {
                if let Some(&value) = row.get(ny) {
                    let neighbor = Point(nx, ny);

                    if visited.contains(&neighbor) {
                        continue;
                    }

                    if value == array[x][y] + 1 {
                        visited.insert(neighbor.clone());

                        if value == 9 {
                            score += 1;
                        } else {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }
    }

    score
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point(usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_multiple_path() {
        let puzzle = vec![
            vec![8, 8, 8, 0, 8, 8, 8],
            vec![8, 8, 8, 1, 8, 8, 8],
            vec![8, 8, 8, 2, 8, 8, 8],
            vec![6, 5, 4, 3, 4, 5, 6],
            vec![7, 1, 1, 1, 1, 1, 7],
            vec![8, 1, 1, 1, 1, 1, 8],
            vec![9, 1, 1, 1, 1, 1, 9],
        ];
        assert_eq!(
            tail_head_from_starting_point_count(&puzzle, &Point(0, 3)),
            2
        );
    }

    #[test]
    fn should_find_one_tail_head_one_horizontal() {
        let puzzle = vec![
            vec![9],
            vec![8],
            vec![7],
            vec![6],
            vec![5],
            vec![4],
            vec![3],
            vec![2],
            vec![1],
            vec![0],
        ];
        assert_eq!(
            tail_head_from_starting_point_count(&puzzle, &Point(9, 0)),
            1
        );
    }

    #[test]
    fn should_find_one_tail_head_one_line_reverse() {
        let puzzle = vec![vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]];
        assert_eq!(
            tail_head_from_starting_point_count(&puzzle, &Point(0, 9)),
            1
        );
    }

    #[test]
    fn should_find_one_tail_head_one_line() {
        let puzzle = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]];
        assert_eq!(
            tail_head_from_starting_point_count(&puzzle, &Point(0, 0)),
            1
        );
    }

    #[test]
    fn should_convert_to_two_dimensional_array() {
        let puzzle = "0123456789\r\n987654321\r\n";
        assert_eq!(
            to_two_dimensional_array(puzzle),
            vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1]
            ]
        );
    }

    #[test]
    fn should_find_starting_points() {
        assert_eq!(
            find_starting_points("01234\n10234\n43210"),
            vec![Point(0, 0), Point(1, 1), Point(2, 4)]
        );
    }

    #[test]
    fn should_find_starting_points_empty_lines() {
        assert_eq!(find_starting_points(""), Vec::<Point>::new());
    }
}
