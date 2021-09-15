//     0  1  2  3 width = 4
// 0 [ 0  1  2  3
//
// 1   4  5  6  7
//
// 2   8  9  10 11
//
// 3   12 13 14 15 ]
// height = 4

// i = width index = 2
// j = height index = 1
// 2darray [2, 1] == 1darray[6]
// 2darray[i, j] = 1darray [width * j + i]
// 2darray[2, 1] = 1darray [4 * 1 + 2]

pub struct TwoDArray<T: Copy> {
    pub data: Box<[T]>,
    pub height: usize,
    pub width: usize,
}

impl<T: Copy> TwoDArray<T> {
    pub fn new(height: usize, width: usize, initial_val: T) -> TwoDArray<T> {
        let vec: Vec<T> = vec![initial_val; height * width];
        TwoDArray {
            height: height,
            width: width,
            data: vec.into_boxed_slice(),
        }
    }

    // i is y axis
    // j is x axis
    pub fn index(&self, i: usize, j: usize) -> Option<usize> {
        if i >= self.height || j >= self.width {
            return None;
        }
        Some(i * self.width + j)
    }

    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        self.index(i, j).map(|idx| self.data[idx])
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) {
        if let Some(idx) = self.index(i, j) {
            self.data[idx] = val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_array() {
        let width = 10;
        let height = 10;
        let mut matrix: TwoDArray<i32> = TwoDArray::new(width, height, 0);

        // test index math
        assert_eq!(matrix.index(2, 0), Some(20));

        // test set/get
        matrix.set(4, 5, 42);
        assert_eq!(matrix.get(4, 5), Some(42));

        // test out of bounds
        assert_eq!(matrix.get(25, 25), None);
        assert_eq!(matrix.index(25, 25), None);
    }
}
