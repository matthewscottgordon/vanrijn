#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub start_column: usize,
    pub end_column: usize,
    pub start_row: usize,
    pub end_row: usize,
}

impl Tile {
    pub fn width(&self) -> usize {
        self.end_column - self.start_column
    }
    pub fn height(&self) -> usize {
        self.end_row - self.start_row
    }
}

#[derive(Clone)]
pub struct TileIterator {
    tile_size: usize,
    total_height: usize,
    total_width: usize,
    current_column: usize,
    current_row: usize,
}

impl TileIterator {
    pub fn new(total_width: usize, total_height: usize, tile_size: usize) -> TileIterator {
        // If tile_size*2 is greater than usize::max_value(), increment would overflow
        assert!(tile_size > 0 && tile_size * 2 < usize::max_value());
        TileIterator {
            tile_size,
            total_width,
            total_height,
            current_column: 0,
            current_row: 0,
        }
    }
}

impl Iterator for TileIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        if self.current_row >= self.total_height {
            None
        } else {
            let start_column = self.current_column;
            let end_column = self.total_width.min(start_column + self.tile_size);
            let start_row = self.current_row;
            let end_row = self.total_height.min(start_row + self.tile_size);

            self.current_column += self.tile_size;
            if self.current_column >= self.total_width {
                self.current_row += self.tile_size;
                self.current_column = 0;
            }

            Some(Tile {
                start_column,
                end_column,
                start_row,
                end_row,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[test]
    fn iterator_has_correct_number_of_tiles_when_dimensions_are_multiple_of_tile_size() {
        let target = TileIterator::new(20, 15, 5);
        assert!(target.count() == 12);
    }

    #[test]
    fn iterator_has_correct_number_of_tiles_when_width_is_one_under_multiple_of_tile_size() {
        let target = TileIterator::new(19, 15, 5);
        assert!(target.count() == 12);
    }

    #[test]
    fn iterator_has_correct_number_of_tiles_when_width_is_one_over_multiple_of_tile_size() {
        let target = TileIterator::new(21, 15, 5);
        assert!(target.count() == 15);
    }

    #[test]
    fn iterator_has_correct_number_of_tiles_when_height_is_one_under_multiple_of_tile_size() {
        let target = TileIterator::new(20, 14, 5);
        assert!(target.count() == 12);
    }

    #[test]
    fn iterator_has_correct_number_of_tiles_when_height_is_one_over_multiple_of_tile_size() {
        let target = TileIterator::new(20, 16, 5);
        assert!(target.count() == 16);
    }

    #[quickcheck]
    fn tiles_are_expected_size(width: usize, height: usize, tile_size: usize) -> TestResult {
        let max_size = 10000;
        // Check size of width and height first, since width*height might overflow.
        if width > max_size || height > max_size || width * height > max_size {
            return TestResult::discard();
        }
        if tile_size == 0 {
            return TestResult::discard();
        }

        let mut target = TileIterator::new(width, height, tile_size);
        TestResult::from_bool(target.all(|tile| {
            tile.end_column - tile.start_column <= tile_size
                && tile.end_row - tile.start_row <= tile_size
        }))
    }

    #[quickcheck]
    fn iterator_includes_all_coordinates_exactly_once(
        width: usize,
        height: usize,
        tile_size: usize,
    ) -> TestResult {
        let max_size = 10000;
        // Check size of width and height first, since width*height might overflow.
        if width > max_size || height > max_size || width * height > max_size {
            return TestResult::discard();
        }
        if tile_size == 0 {
            return TestResult::discard();
        }

        let target = TileIterator::new(width, height, tile_size);
        let mut index_counts = vec![0; width * height];
        let mut total_count = 0;
        for tile in target {
            for column in tile.start_column..tile.end_column {
                for row in tile.start_row..tile.end_row {
                    index_counts[row * width + column] += 1;
                    total_count += 1;
                    if total_count > width * height {
                        return TestResult::failed();
                    }
                }
            }
        }
        TestResult::from_bool(index_counts.iter().all(|&elem| elem == 1))
    }
}
