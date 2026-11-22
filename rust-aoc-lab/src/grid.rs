//! A rectangular grid of bytes, stored in one flat `Vec`.

/// A rectangle of ASCII bytes.
///
/// The cells live in one `Vec`, row by row. The index of row `r` and column `c`
/// is `r * width + c`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub cells: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    /// Read a rectangular grid from consecutive non-empty lines.
    ///
    /// Every line must have the same length. A trailing newline is fine.
    pub fn parse(input: &str) -> Self {
        let rows: Vec<&str> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        let height = rows.len();
        let width = rows.first().map_or(0, |row| row.len());
        let mut cells = Vec::with_capacity(width * height);

        for row in &rows {
            assert_eq!(row.len(), width, "every grid row must have the same length");
            cells.extend_from_slice(row.as_bytes());
        }

        Self {
            cells,
            width,
            height,
        }
    }

    /// The flat index of a row and column that are known to be inside.
    pub fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    /// The row and column of a flat index.
    pub fn row_col(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    /// The byte at a row and column that are known to be inside.
    pub fn at(&self, row: usize, col: usize) -> u8 {
        self.cells[self.idx(row, col)]
    }

    /// Change one cell.
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        let index = self.idx(row, col);
        self.cells[index] = value;
    }

    /// The byte at a possibly negative position, or `None` outside the grid.
    pub fn get(&self, row: i32, col: i32) -> Option<u8> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;
        if row >= self.height || col >= self.width {
            return None;
        }
        Some(self.at(row, col))
    }

    /// The flat index of the first cell holding `needle`.
    pub fn find(&self, needle: u8) -> Option<usize> {
        self.cells.iter().position(|&cell| cell == needle)
    }

    /// The flat indices of the four orthogonal neighbours inside the grid.
    pub fn neighbors4(&self, index: usize) -> Vec<usize> {
        let (row, col) = self.row_col(index);
        let mut neighbors = Vec::with_capacity(4);

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if self.get(row as i32 + dr, col as i32 + dc).is_some() {
                let next_row = (row as i32 + dr) as usize;
                let next_col = (col as i32 + dc) as usize;
                neighbors.push(self.idx(next_row, next_col));
            }
        }

        neighbors
    }

    /// The flat indices of the eight surrounding cells inside the grid.
    pub fn neighbors8(&self, index: usize) -> Vec<usize> {
        let (row, col) = self.row_col(index);
        let mut neighbors = Vec::with_capacity(8);

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if self.get(row as i32 + dr, col as i32 + dc).is_some() {
                    let next_row = (row as i32 + dr) as usize;
                    let next_col = (col as i32 + dc) as usize;
                    neighbors.push(self.idx(next_row, next_col));
                }
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Grid {
        Grid::parse("..#\n.#.\n#..\n")
    }

    #[test]
    fn parse_keeps_width_and_height() {
        let grid = example();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.cells.len(), 9);
    }

    #[test]
    fn idx_and_row_col_agree() {
        let grid = example();
        assert_eq!(grid.idx(1, 2), 5);
        assert_eq!(grid.row_col(5), (1, 2));
    }

    #[test]
    fn get_rejects_negative_positions() {
        let grid = example();
        assert_eq!(grid.get(-1, 0), None);
        assert_eq!(grid.get(0, -1), None);
        assert_eq!(grid.get(0, 0), Some(b'.'));
    }

    #[test]
    fn a_corner_has_two_neighbors() {
        let grid = example();
        assert_eq!(grid.neighbors4(0), vec![3, 1]);
    }

    #[test]
    fn a_corner_has_three_diagonal_neighbors() {
        let grid = example();
        assert_eq!(grid.neighbors8(0), vec![1, 3, 4]);
    }

    #[test]
    fn find_returns_the_first_match() {
        let grid = example();
        assert_eq!(grid.find(b'#'), Some(2));
    }
}
