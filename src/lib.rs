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
}
