use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

mod paper;
use crate::paper::PaperHouse;

use std::thread;

mod file_parser;
use crate::file_parser::{FileParser, FileParserTrait};

pub fn solve(input_file: &str) -> (u64, u64) {
    let input_grid = FileParser::new(input_file).parse_grid();
    let mut warehouse = PaperHouse::new(input_grid);

    let part_1 = warehouse.remove_available_paper_rolls();
    let mut paper_rolls_removed_this_go = part_1;
    let mut part_2 = part_1;

    while paper_rolls_removed_this_go != 0 {
        paper_rolls_removed_this_go = warehouse.remove_available_paper_rolls();
        part_2 += paper_rolls_removed_this_go;
    }

    (part_1, part_2)
}

pub fn load_file(input_file: &str) -> Vec<Vec<char>> {
    FileParser::new(input_file).parse_grid()
}

pub fn solve_part1(input_grid: Vec<Vec<char>>) -> u64 {
    let mut warehouse = PaperHouse::new(input_grid);

    warehouse.remove_available_paper_rolls()
}

pub fn solve_part2(input_grid: Vec<Vec<char>>) -> u64 {
    let mut warehouse = PaperHouse::new(input_grid);

    let part_1 = warehouse.remove_available_paper_rolls();
    let mut paper_rolls_removed_this_go = part_1;
    let mut part_2 = part_1;

    while paper_rolls_removed_this_go != 0 {
        paper_rolls_removed_this_go = warehouse.remove_available_paper_rolls();
        part_2 += paper_rolls_removed_this_go;
    }
    part_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (part_1, part_2) = solve("data/example.txt");
        assert_eq!(part_1, 13);
        assert_eq!(part_2, 43);
    }

    #[test]
    fn actual() {
        let (part_1, part_2) = solve("data/input.txt");
        assert_eq!(part_1, 1384);
        assert_eq!(part_2, 8013);
    }

    #[test]
    fn actual_parts() {
        let input_file = load_file("data/input.txt");
        assert_eq!(solve_part1(input_file), 1384);
        let input_file = load_file("data/input.txt");
        assert_eq!(solve_part2(input_file), 8013);
    }
}
