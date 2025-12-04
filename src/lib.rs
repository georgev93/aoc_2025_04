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
    // let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    // let result1 = Arc::new(AtomicU64::new(0));
    // let result2 = Arc::new(AtomicU64::new(0));

    // let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(battery_banks.len());
    // for battery_bank in battery_banks {
    //     let result1_clone = Arc::clone(&result1);
    //     let result2_clone = Arc::clone(&result2);
    //     let handle = thread::spawn(move || {
    //         result1_clone.fetch_add(battery_bank.get_high_joltage(2), Ordering::SeqCst);
    //         result2_clone.fetch_add(battery_bank.get_high_joltage(12), Ordering::SeqCst);
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().expect("Thread panicked!");
    // }
    //
    // (
    //     result1.load(Ordering::Relaxed),
    //     result2.load(Ordering::Relaxed),
    // )
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
        assert_eq!(part_2, 0);
    }
}
