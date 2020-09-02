use std::ops::{Index, IndexMut};

/// 3D row-major dynamic array
#[derive(Debug)]
pub struct Array2D<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T: Default + Clone> Array2D<T> {
    /// Create Array2D with `width` and `height` filled with default values.
    pub fn new(height: usize, width: usize) -> Array2D<T> {
        Array2D {
            data: vec![Default::default(); width * height],
            height,
            width,
        }
    }

    /// Reset contents of array to all default values
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Array2D<T> {
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Return single slice containing all elements (row-major)
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T: Copy> Array2D<T> {
    /// Copy `source` into the Array2D at the specified location
    pub fn update_block(&mut self, start_row: usize, start_column: usize, source: &Array2D<T>) {
        let end_row = start_row + source.height;
        let end_column = start_column + source.width;
        assert!(end_row <= self.height);
        for i in 0..source.height {
            self[start_row + i][start_column..end_column].copy_from_slice(&source[i])
        }
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        assert!(index < self.height);
        &self.data[(index * self.width)..((index + 1) * self.width)]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        assert!(index < self.height);
        &mut self.data[(index * self.width)..((index + 1) * self.width)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_u8_is_all_zeros() {
        let target: Array2D<u8> = Array2D::new(10, 12);
        for i in 0..10 {
            for j in 0..12 {
                assert!(target[i][j] == 0);
            }
        }
    }

    #[test]
    #[should_panic]
    fn panics_if_row_outside_bounds() {
        let target: Array2D<u8> = Array2D::new(10, 12);
        assert!(target[10][6] == 0);
    }

    #[test]
    #[should_panic]
    fn panics_if_column_outside_bounds() {
        let target: Array2D<u8> = Array2D::new(10, 12);
        assert!(target[5][12] == 0);
    }

    #[test]
    fn write_and_read_all_preserves_values() {
        let mut target: Array2D<u8> = Array2D::new(10, 12);
        for i in 0..10 {
            for j in 0..12 {
                target[i][j] = (i * 10 + j) as u8;
            }
        }
        for i in 0..10 {
            for j in 0..12 {
                assert!(target[i][j] == (i * 10 + j) as u8);
            }
        }
    }

    #[test]
    fn update_block_writes_expected_values_in_block() {
        let mut target: Array2D<u8> = Array2D::new(10, 12);
        let mut source: Array2D<u8> = Array2D::new(2, 3);
        for i in 0..2 {
            for j in 0..3 {
                source[i][j] = (i * 3 + j) as u8;
            }
        }
        target.update_block(4, 5, &source);
        dbg!(&target);
        for i in 0..2 {
            for j in 0..3 {
                assert!(dbg!(target[4 + i][5 + j]) == dbg!(source[i][j]));
            }
        }
    }
}
