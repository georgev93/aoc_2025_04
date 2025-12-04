pub struct PaperHouse {
    layout: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

struct Coord {
    x: usize,
    y: usize,
}

impl PaperHouse {
    pub fn new(input_grid: Vec<Vec<char>>) -> Self {
        Self {
            width: input_grid[0].len(),
            height: input_grid.len(),
            layout: input_grid,
        }
    }

    pub fn remove_available_paper_rolls(&mut self) -> u64 {
        let mut ret_val = 0u64;
        let mut coords_to_be_removed: Vec<Coord> = Vec::new();

        for row in 0..self.height {
            let mut row_string = String::new();
            for col in 0..self.width {
                if (self.get_number_of_paper_neighbors(col, row) < 4)
                    && (self.layout[row][col] == '@')
                {
                    ret_val += 1;
                    coords_to_be_removed.push(Coord { x: col, y: row });
                    row_string.push('x');
                } else {
                    row_string.push(self.layout[row][col]);
                }
                // row_string.push((self.get_number_of_paper_neighbors(col, row) + 48) as char);
            }
            println!("{} {ret_val}", row_string);
        }
        for coord in coords_to_be_removed {
            self.layout[coord.y][coord.x] = '.';
        }
        println!("");
        ret_val
    }

    pub fn get_number_of_paper_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut rows_to_examine: Vec<usize> = vec![];
        let mut cols_to_examine: Vec<usize> = vec![];

        // println!("Running on {x}, {y}");
        // dbg!(&self.layout);

        match x {
            0 => {
                cols_to_examine.push(0);
                cols_to_examine.push(1);
            }
            max if x == (self.width - 1) => {
                cols_to_examine.push(max - 1);
                cols_to_examine.push(max);
            }
            _ => {
                cols_to_examine.push(x - 1);
                cols_to_examine.push(x);
                cols_to_examine.push(x + 1);
            }
        }

        match y {
            0 => {
                rows_to_examine.push(0);
                rows_to_examine.push(1);
            }
            max if y == (self.height - 1) => {
                rows_to_examine.push(max - 1);
                rows_to_examine.push(max);
            }
            _ => {
                rows_to_examine.push(y - 1);
                rows_to_examine.push(y);
                rows_to_examine.push(y + 1);
            }
        }

        let mut paper_count = 0;
        for row in &rows_to_examine {
            for col in &cols_to_examine {
                if (*col != x) || (*row != y) {
                    // println!("Checking coordinate {col}, {row}");
                    if self.layout[*row][*col] == '@' {
                        // println!("Paper found there!");
                        paper_count += 1;
                    }
                }
            }
        }
        paper_count
    }
}

#[cfg(test)]
pub mod mock_grid {
    pub fn str_into_grid(input: &str) -> Vec<Vec<char>> {
        input
            .split("\n")
            .map(|line| line.trim_ascii())
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes().iter().map(|b| *b as char).collect())
            .collect()
    }
}
mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        let warehouse_grid = mock_grid::str_into_grid(
            "
            @@
            ..
            @.
            ",
        );
        let my_warehouse = PaperHouse::new(warehouse_grid);
        assert_eq!(my_warehouse.layout[0], vec!['@', '@']);
        assert_eq!(my_warehouse.layout[1], vec!['.', '.']);
        assert_eq!(my_warehouse.layout[2], vec!['@', '.']);
        assert_eq!(my_warehouse.layout[2][0], '@');
        assert_eq!(my_warehouse.width, 2);
        assert_eq!(my_warehouse.height, 3);
    }

    #[test]
    fn paper_count() {
        let warehouse_grid = mock_grid::str_into_grid(
            " ..@@.@@@@.
              @@@.@.@.@@
              @@@@@.@.@@
              @.@@@@..@.
              @@.@@@@.@@
              .@@@@@@@.@
              .@.@.@.@@@
              @.@@@.@@@@
              .@@@@@@@@.
              @.@.@@@.@.",
        );
        let my_warehouse = PaperHouse::new(warehouse_grid);
        assert_eq!(my_warehouse.get_number_of_paper_neighbors(0, 0), 2);
        assert_eq!(my_warehouse.get_number_of_paper_neighbors(9, 0), 3);
        assert_eq!(my_warehouse.get_number_of_paper_neighbors(9, 9), 2);
        assert_eq!(my_warehouse.get_number_of_paper_neighbors(0, 9), 1);
        assert_eq!(my_warehouse.get_number_of_paper_neighbors(1, 5), 4);
        assert_eq!(my_warehouse.get_number_of_paper_neighbors(6, 0), 3);
    }
}
