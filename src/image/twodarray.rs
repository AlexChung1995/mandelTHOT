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
        height: usize,
        width: usize,
    }

    impl<T: Copy> TwoDArray<T> {
        pub fn new(height: usize, width: usize, initial_val: T) -> TwoDArray<T> {
            let vec: Vec<T> = vec![initial_val; height * width];
            return TwoDArray {
                height: height,
                width: width,
                data: vec.into_boxed_slice(),
            }
        }

        pub fn index(&self, i: usize, j: usize) -> usize {
            return j * self.width + i;
        }

        pub fn get(&self, i: usize, j: usize) -> T {
            let index = self.index(i, j);
            return self.data[index];
        }

        pub fn set(&mut self, i: usize, j: usize, val: T) {
            let index = self.index(i, j);
            self.data[index] = val;
        }
    }

