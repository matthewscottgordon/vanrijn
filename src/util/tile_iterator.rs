pub struct Tile {
    pub start_x: usize,
    pub end_x: usize,
    pub start_y: usize,
    pub end_y: usize,
}

pub struct TileIterator {
    tile_size: usize,
    total_height: usize,
    total_width: usize,
    current_x: usize,
    current_y: usize,
}

impl TileIterator {
    pub fn new(total_width: usize, total_height: usize, tile_size: usize) -> TileIterator {
        // If tile_size*2 is greater than usize::max_value(), increment would overflow
        assert!(tile_size > 0 && tile_size*2 < usize::max_value());
        TileIterator {
            tile_size,
            total_width,
            total_height,
            current_x: 0,
            current_y: 0,
        }
    }
}

impl Iterator for TileIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        if self.current_y >= self.total_height {
            None
        } else {
            let start_x = self.current_x;
            let end_x = self.total_width.min(start_x + self.tile_size);
            let start_y = self.current_y;
            let end_y = self.total_height.min(start_y + self.tile_size);

            self.current_x += self.tile_size;
            if self.current_x >= self.total_width {
                self.current_y += self.tile_size;
                self.current_x = 0;
            }

            Some(Tile {
                start_x,
                end_x,
                start_y,
                end_y,
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
        // Check size of width and height first, since width*height my overflow.
        if width > max_size || height > max_size || width * height > max_size {
            return TestResult::discard();
        }
        if tile_size == 0 {
            return TestResult::discard();
        }

        let mut target = TileIterator::new(width, height, tile_size);
        TestResult::from_bool(target.all(|tile| {
            tile.end_x - tile.start_x <= tile_size && tile.end_y - tile.start_y <= tile_size
        }))
    }

    #[quickcheck]
    fn iterator_includes_all_coordinates_exactly_once(
        width: usize,
        height: usize,
        tile_size: usize,
    ) -> TestResult {
        let max_size = 10000;
        // Check size of width and height first, since width*height my overflow.
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
            for x in tile.start_x..tile.end_x {
                for y in tile.start_y..tile.end_y {
                    index_counts[y * width + x] += 1;
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
